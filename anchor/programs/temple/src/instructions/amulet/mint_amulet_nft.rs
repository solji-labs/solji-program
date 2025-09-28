use crate::error::ErrorCode;
use crate::state::amulet::{AmuletNFT, AmuletSource};
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::UserState;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

// TODO 抽签/许愿  可概率获得 ？前端去处理概率吗
pub fn mint_amulet_nft(ctx: Context<MintAmuletNFT>, source: u8) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // 检查寺庙状态
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::MintNFT,
        current_time,
    )?;

    // 检查用户是否有足够的pending_amulets余额
    require!(
        ctx.accounts.user_state.pending_amulets > 0,
        crate::error::ErrorCode::InsufficientPendingAmulets
    );

    // 消耗一个pending_amulets
    ctx.accounts.user_state.pending_amulets -= 1;

    // 获取序列号
    let serial_number = ctx.accounts.temple_config.total_amulets;

    let nft_name_str = format!("御守 #{}", serial_number);
    let nft_description = match source {
        0 => "通过抽签获得的幸运御守",
        1 => "通过许愿获得的祝福御守",
        _ => "获得的御守",
    };

    // 创建元数据账户
    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                mint_authority: ctx.accounts.temple_config.to_account_info(),
                update_authority: ctx.accounts.temple_config.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            temple_signer_seeds,
        ),
        DataV2 {
            name: nft_name_str.clone(),
            symbol: "AMULET".to_string(),
            uri: format!(
                "https://api.foxverse.co/temple/amulet/{}/metadata.json",
                serial_number
            ),
            seller_fee_basis_points: 500, // 5% 版税
            creators: None,
            collection: None,
            uses: None,
        },
        false, // 不可变
        true,
        None,
    )?;

    // Mint 御守NFT
    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.temple_config.to_account_info(),
            },
            temple_signer_seeds,
        ),
        1,
    )?;
    msg!("Amulet NFT minted successfully");

    // 初始化AmuletNFT账户数据
    ctx.accounts.amulet_nft_account.owner = ctx.accounts.authority.key();
    ctx.accounts.amulet_nft_account.mint = ctx.accounts.nft_mint_account.key();
    ctx.accounts.amulet_nft_account.name = nft_name_str.clone();
    ctx.accounts.amulet_nft_account.description = nft_description.to_string();
    ctx.accounts.amulet_nft_account.minted_at = clock.unix_timestamp;
    ctx.accounts.amulet_nft_account.source = source;
    ctx.accounts.amulet_nft_account.serial_number = serial_number as u32;

    // 更新寺庙配置
    ctx.accounts.temple_config.total_amulets += 1;

    // 更新全局统计
    ctx.accounts.global_stats.increment_amulets();

    Ok(())
}

#[derive(Accounts)]
pub struct MintAmuletNFT<'info> {
    /// 用户账号
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        mut,
        seeds = [GlobalStats::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub global_stats: Account<'info, GlobalStats>,

    /// 用户账号
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        init,
        payer = authority,
        seeds = [
            AmuletNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            authority.key().as_ref(),
            &[temple_config.total_amulets as u8], // 使用当前总数作为序列号种子
        ],
        bump,
        mint::decimals = AmuletNFT::TOKEN_DECIMALS,
        mint::authority = temple_config.key(),
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// 用户的NFT关联账户
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + AmuletNFT::INIT_SPACE,
        seeds = [AmuletNFT::SEED_PREFIX.as_bytes(), b"account", temple_config.key().as_ref(), authority.key().as_ref(), &[temple_config.total_amulets as u8]],
        bump
    )]
    pub amulet_nft_account: Account<'info, AmuletNFT>,

    /// CHECK: this is the metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub meta_account: UncheckedAccount<'info>,

    // 程序账号
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
