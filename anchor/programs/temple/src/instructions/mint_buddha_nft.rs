use crate::error::ErrorCode;
use crate::state::buddha_nft::*;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::*;
use crate::state::user_state::*;
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

pub fn mint_buddha_nft(ctx: Context<MintBuddhaNFT>) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // 检查寺庙状态
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::MintNFT,
        current_time,
    )?;

    // 供应量
    require!(
        ctx.accounts.temple_config.total_buddha_nft < 10000,
        ErrorCode::BuddhaNFTSupplyExceeded
    );
    // 检查用户是否已经拥有佛像
    require!(
        !ctx.accounts.user_state.has_buddha_nft,
        ErrorCode::UserHasBuddhaNFT
    );

    require!(
        ctx.accounts.user_donate_state.can_mint_buddha_free(),
        ErrorCode::InsufficientDonation
    );

    // 铸造佛像
    let temple_config_key = ctx.accounts.temple_config.key();
    let authority_key = ctx.accounts.authority.key();
    let serial_number = ctx.accounts.temple_config.total_buddha_nft;

    let nft_name = format!(
        "Buddha NFT #{}",
        ctx.accounts.temple_config.total_buddha_nft
    );

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
                update_authority: ctx.accounts.authority.to_account_info(),
                payer: ctx.accounts.temple_treasury.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            temple_signer_seeds,
        ),
        DataV2 {
            name: nft_name,
            symbol: BuddhaNFT::TOKEN_SYMBOL.to_string(),
            uri: BuddhaNFT::TOKEN_URL.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, // 不可变
        true,
        None,
    )?;

    // Mint 佛像
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
    msg!("NFT minted successfully");

    // 初始化BuddhaNFT账户数据
    ctx.accounts.buddha_nft_account.owner = ctx.accounts.authority.key();
    ctx.accounts.buddha_nft_account.mint = ctx.accounts.nft_mint_account.key();
    ctx.accounts.buddha_nft_account.serial_number = serial_number as u32;
    ctx.accounts.buddha_nft_account.minted_at = Clock::get()?.unix_timestamp;
    ctx.accounts.buddha_nft_account.is_active = true;

    // 冻结NFT代币账户，防止转让
    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    anchor_spl::token::freeze_account(CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        anchor_spl::token::FreezeAccount {
            account: ctx.accounts.nft_associated_token_account.to_account_info(),
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            authority: ctx.accounts.temple_config.to_account_info(),
        },
        temple_signer_seeds,
    ))?;

    // 更新用户状态
    ctx.accounts.user_state.has_buddha_nft = true;

    // 更新寺庙配置
    ctx.accounts.temple_config.total_buddha_nft += 1;

    // 更新全局统计
    ctx.accounts.global_stats.increment_buddha_lights();

    Ok(())
}

#[derive(Accounts)]
pub struct MintBuddhaNFT<'info> {
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

    /// CHECK: 寺庙国库账户
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,

    /// 用户账号
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    /// 捐助账号
    #[account(
        mut,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_donate_state: Box<Account<'info, UserDonationState>>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [
            BuddhaNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            authority.key().as_ref()
        ],
        bump,
        mint::decimals = BuddhaNFT::TOKEN_DECIMALS,
        mint::authority = temple_config.key(),
        mint::freeze_authority = temple_config.key(),
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
        space = 8 +  BuddhaNFT::INIT_SPACE,
        seeds = [BuddhaNFT::SEED_PREFIX.as_bytes(), b"account", temple_config.key().as_ref(), authority.key().as_ref()],
        bump
    )]
    pub buddha_nft_account: Account<'info, BuddhaNFT>,

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
