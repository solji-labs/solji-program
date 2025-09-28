use crate::error::ErrorCode;
use crate::state::event::DonationNFTMinted;
use crate::state::global_stats::GlobalStats;
use crate::state::medal_nft::*;
use crate::state::temple_config::*;
use crate::state::user_state::{UserDonationState, UserIncenseState, UserState};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::update_metadata_accounts_v2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::metadata::UpdateMetadataAccountsV2;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
/// 勋章
/// 捐助后调用 会进行更新或者新mint 勋章nft
pub fn mint_medal_nft(ctx: Context<MintMedalNFT>) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // 检查寺庙状态
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::MintNFT,
        current_time,
    )?;

    let user_state = &mut ctx.accounts.user_state;
    let donation_sol = ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0;

    // 检查用户是否已有勋章NFT
    if user_state.has_medal_nft {
        // 用户已有勋章，检查是否可以升级
        let next_upgrade_level = ctx
            .accounts
            .medal_nft_account
            .get_next_upgrade_level(donation_sol);
        if let Some(new_level) = next_upgrade_level {
            // 构建更新后的元数据
            let serial_number = ctx.accounts.medal_nft_account.serial_number;
            let new_name = if new_level == 4 {
                format!("至尊龙章 #{}", serial_number)
            } else if new_level == 3 {
                format!("护法金章 #{}", serial_number)
            } else if new_level == 2 {
                format!("精进银章 #{}", serial_number)
            } else {
                format!("入门功德铜章 #{}", serial_number)
            };

            let new_uri = format!(
                "https://api.foxverse.co/temple/medal/{}/metadata.json",
                new_level
            );

            // 使用update_metadata_accounts_v2更新元数据
            update_metadata_accounts_v2(
                CpiContext::new(
                    ctx.accounts.token_metadata_program.to_account_info(),
                    UpdateMetadataAccountsV2 {
                        metadata: ctx.accounts.meta_account.to_account_info(),
                        update_authority: ctx.accounts.authority.to_account_info(),
                    },
                ),
                None, // 保持现有的 update_authority
                Some(DataV2 {
                    name: new_name.clone(),
                    symbol: "TMM".to_string(),
                    uri: new_uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                }),
                None,
                if new_level == 4 { Some(true) } else { None }, // 只有最高等级才设置为不可变
            )?;

            // 更新MedalNFT账户数据
            let now = Clock::get()?.unix_timestamp;
            ctx.accounts.medal_nft_account.level = new_level;
            ctx.accounts.medal_nft_account.total_donation =
                ctx.accounts.user_donation_state.donation_amount;
            ctx.accounts.medal_nft_account.last_upgrade = now;

            msg!("寺庙勋章NFT升级成功: {}", new_name);
            msg!("新等级: {}", new_level);
            msg!("总捐款金额: {:.6} SOL", donation_sol);

            // 发出NFT铸造事件
            emit!(DonationNFTMinted {
                user: ctx.accounts.authority.key(),
                nft_mint: ctx.accounts.nft_mint_account.key(),
                level: new_level,
                serial_number,
                timestamp: clock.unix_timestamp,
            });
        } else {
            msg!("当前捐助金额不足以升级勋章NFT");
            return err!(ErrorCode::InsufficientDonationForUpgrade);
        }
    } else {
        // 铸造新NFT
        // 检查用户是否达到捐款等级要求
        if donation_sol < MedalNFT::get_level_min_donation_sol(1) {
            return err!(ErrorCode::InsufficientDonationForMedal);
        }

        // 确定用户当前的等级
        let mut current_level = 1;
        for level in (1..=4).rev() {
            if donation_sol >= MedalNFT::get_level_min_donation_sol(level) {
                current_level = level;
                break;
            }
        }

        // 生成序列号
        let serial_number = ctx.accounts.user_donation_state.total_donation_count;

        let medal_name = if current_level == 4 {
            format!("至尊龙章 #{}", serial_number)
        } else if current_level == 3 {
            format!("护法金章 #{}", serial_number)
        } else if current_level == 2 {
            format!("精进银章 #{}", serial_number)
        } else {
            format!("入门功德铜章 #{}", serial_number)
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
                    update_authority: ctx.accounts.authority.to_account_info(),
                    payer: ctx.accounts.authority.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                temple_signer_seeds,
            ),
            DataV2 {
                name: medal_name.clone(),
                symbol: "TMM".to_string(),
                uri: format!(
                    "https://api.foxverse.co/temple/medal/{}/metadata.json",
                    current_level
                ),
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true, // 允许元数据可变，以便后续升级
            true,
            None,
        )?;

        // 铸造勋章NFT
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

        // 初始化MedalNFT账户数据
        let now = Clock::get()?.unix_timestamp;
        ctx.accounts.medal_nft_account.owner = ctx.accounts.authority.key();
        ctx.accounts.medal_nft_account.mint = ctx.accounts.nft_mint_account.key();
        ctx.accounts.medal_nft_account.level = current_level;
        ctx.accounts.medal_nft_account.total_donation =
            ctx.accounts.user_donation_state.donation_amount;
        ctx.accounts.medal_nft_account.minted_at = now;
        ctx.accounts.medal_nft_account.last_upgrade = now;
        ctx.accounts.medal_nft_account.merit = ctx.accounts.user_incense_state.merit;
        ctx.accounts.medal_nft_account.serial_number = serial_number;

        // 更新用户状态
        ctx.accounts.user_state.has_medal_nft = true;

        // 更新全局统计
        ctx.accounts.global_stats.increment_fortune_nfts();

        msg!("寺庙勋章NFT铸造成功: {}", medal_name);
        msg!("勋章等级: {}", current_level);
        msg!("总捐款金额: {:.6} SOL", donation_sol);

        // 发出NFT铸造事件
        emit!(DonationNFTMinted {
            user: ctx.accounts.authority.key(),
            nft_mint: ctx.accounts.nft_mint_account.key(),
            level: current_level,
            serial_number,
            timestamp: clock.unix_timestamp,
        });
    }

    Ok(())
}

#[derive(Accounts)]
pub struct MintMedalNFT<'info> {
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

    /// 用户状态账户
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    /// 用户捐赠状态账户
    #[account(
        mut,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    /// 用户香火状态账户
    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + MedalNFT::INIT_SPACE,
        seeds = [MedalNFT::SEED_PREFIX.as_bytes(), b"account", temple_config.key().as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub medal_nft_account: Box<Account<'info, MedalNFT>>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [
            MedalNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            authority.key().as_ref()
        ],
        bump,
        mint::decimals = MedalNFT::TOKEN_DECIMALS,
        mint::authority = temple_config.key(),
        mint::freeze_authority = temple_config.key(),
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// 用户的勋章NFT关联账户
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

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
