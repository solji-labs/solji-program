use crate::error::ErrorCode;
use crate::state::global_stats::GlobalStats;
use crate::state::medal_nft::*;
use crate::state::temple_config::TempleConfig;
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
/// ⚠️废弃 已拆分成多个指令 这个指令太大了
#[derive(Accounts)]
pub struct Donate<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,

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

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump = user_state.bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        mut,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    /// CHECK: 寺庙国库账户
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,

    // medal NFT 相关账户
    #[account(
        init_if_needed,
        payer = donor,
        space = 8 + MedalNFT::INIT_SPACE,
        seeds = [MedalNFT::SEED_PREFIX.as_bytes(),b"account",  temple_config.key().as_ref(), donor.key().as_ref()],
        bump,
    )]
    pub medal_nft_account: Box<Account<'info, MedalNFT>>,

    #[account(
        init_if_needed,
        payer = donor,
        seeds = [
            MedalNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            donor.key().as_ref()
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
        payer = donor,
        associated_token::mint = nft_mint_account,
        associated_token::authority = donor,
    )]
    pub nft_associated_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: this is the metadata account - must be writable for metadata creation/updates
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

pub fn donate(mut ctx: Context<Donate>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // 检查寺庙状态
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::Donate,
        current_time,
    )?;

    let donor = &ctx.accounts.donor;
    let temple_treasury = &ctx.accounts.temple_treasury;

    // 验证捐助金额
    require!(amount > 0, ErrorCode::InvalidAmount);
    require!(amount >= 1000, ErrorCode::InvalidAmount); // 最少0.000001 SOL

    // 转账SOL到寺庙国库
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &donor.key(),
        &temple_treasury.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            donor.to_account_info(),
            temple_treasury.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // 处理捐助逻辑
    ctx.accounts.user_donation_state.process_donation(amount);

    // 获取捐助等级奖励
    let (merit_reward, incense_points_reward) =
        ctx.accounts.user_donation_state.get_donation_rewards();

    // 更新用户香火状态
    if merit_reward > 0 || incense_points_reward > 0 {
        ctx.accounts
            .user_incense_state
            .add_incense_value_and_merit(incense_points_reward, merit_reward);
        msg!(
            "捐助获得奖励 - 功德值: {}, 香火值: {}",
            merit_reward,
            incense_points_reward
        );
    }

    // 更新全局统计
    ctx.accounts.global_stats.add_donation(amount);
    ctx.accounts
        .global_stats
        .add_incense_value_and_merit(incense_points_reward, merit_reward);

    // 处理捐助解锁香逻辑 - 从动态配置读取
    let donation_sol = amount as f64 / 1_000_000_000.0;
    let total_donation_sol =
        ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0;

    // 捐助关联烧香奖励
    for reward_config in &ctx.accounts.temple_config.dynamic_config.donation_rewards {
        // 检查是否达到最低捐助金额门槛
        if total_donation_sol >= reward_config.min_donation_sol {
            let previous_donation_sol = total_donation_sol - donation_sol;

            // 计算当前捐助后应该获得的奖励总量
            let current_reward = if reward_config.burn_bonus_per_001_sol > 0 {
                // 烧香次数奖励：每0.01SOL增加的烧香次数
                ((total_donation_sol * 100.0) as u64)
                    .saturating_mul(reward_config.burn_bonus_per_001_sol)
            } else {
                // 香奖励：基于门槛的累积奖励
                let current_tier = (total_donation_sol / reward_config.min_donation_sol) as u64;
                current_tier.saturating_mul(reward_config.incense_amount)
            };

            // 计算之前捐助应该获得的奖励总量
            let previous_reward = if reward_config.burn_bonus_per_001_sol > 0 {
                ((previous_donation_sol * 100.0) as u64)
                    .saturating_mul(reward_config.burn_bonus_per_001_sol)
            } else {
                let previous_tier = (previous_donation_sol / reward_config.min_donation_sol) as u64;
                previous_tier.saturating_mul(reward_config.incense_amount)
            };

            // 计算新增的奖励
            let new_reward = current_reward.saturating_sub(previous_reward);

            if new_reward > 0 {
                if reward_config.burn_bonus_per_001_sol > 0 {
                    // 烧香次数奖励
                    ctx.accounts.user_incense_state.incense_number = ctx
                        .accounts
                        .user_incense_state
                        .incense_number
                        .saturating_add(new_reward as u8);
                    msg!("捐助获得额外烧香次数: {}", new_reward);
                } else {
                    // 香奖励
                    ctx.accounts
                        .user_incense_state
                        .add_incense_balance(reward_config.incense_id, new_reward);
                    msg!(
                        "捐助解锁香类型{}: {} 根",
                        reward_config.incense_id,
                        new_reward
                    );
                }
            }
        }
    }

    // 处理勋章 NFT
    let current_donation_sol =
        ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0;

    if !ctx.accounts.user_state.has_medal_nft {
        if current_donation_sol >= MedalNFT::get_level_min_donation_sol(1) {
            // 确定当前等级
            let mut current_level = 1;
            for level in (1..=4).rev() {
                if current_donation_sol >= MedalNFT::get_level_min_donation_sol(level) {
                    current_level = level;
                    break;
                }
            }

            // 使用用户捐赠计数作为序列号
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
                        update_authority: ctx.accounts.donor.to_account_info(),
                        payer: ctx.accounts.donor.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                        rent: ctx.accounts.rent.to_account_info(),
                    },
                    temple_signer_seeds,
                ),
                DataV2 {
                    name: medal_name.clone(),
                    symbol: "TMM".to_string(),
                    uri: format!(
                        "https://api.foxverse.co/temple/medal/{}/metadata.json", // todo
                        current_level
                    ),
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                true, // 用于升级
                true,
                None,
            )?;

            // 铸造NFT
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

            // 勋章NFT可以转让，不需要冻结

            // 初始化MedalNFT账户数据
            let now = Clock::get()?.unix_timestamp;
            ctx.accounts.medal_nft_account.owner = donor.key();
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

            msg!("寺庙勋章NFT铸造成功: {}", medal_name);
            msg!("勋章等级: {}", current_level);
            msg!("总捐款金额: {:.6} SOL", current_donation_sol);
        }
    } else {
    }

    // 记录捐助事件
    let donation_sol = amount as f64 / 1_000_000_000.0;
    msg!("用户 {} 捐助了 {:.6} SOL", donor.key(), donation_sol);
    msg!(
        "当前捐助等级: {}",
        ctx.accounts.user_donation_state.donation_level
    );
    msg!(
        "累计捐助金额: {:.6} SOL",
        ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0
    );

    // 检查佛像mint资格
    if ctx.accounts.user_donation_state.can_mint_buddha_free()
        && !ctx.accounts.user_state.has_buddha_nft
    {
        msg!("恭喜！您已获得免费mint佛像的资格！");
    }

    Ok(())
}
