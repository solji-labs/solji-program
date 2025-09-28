use crate::error::ErrorCode;
use crate::state::donation_leaderboard::DonationLeaderboard;
use crate::state::event::DonationCompleted;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::UserDonationState;
use anchor_lang::prelude::*;

/// 核心捐助资金指令
/// 只处理资金转账和基础记录，发出事件供后续处理

#[derive(Accounts)]
pub struct DonateFund<'info> {
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
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    #[account(
        init_if_needed,
        payer = donor,
        seeds = [DonationLeaderboard::SEED_PREFIX.as_bytes()],
        bump,
        space = 8 + DonationLeaderboard::INIT_SPACE,
    )]
    pub donation_leaderboard: Box<Account<'info, DonationLeaderboard>>,

    /// CHECK: 寺庙国库账户
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn donate_fund(ctx: Context<DonateFund>, amount: u64) -> Result<()> {
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

    // 处理捐助记录
    ctx.accounts.user_donation_state.process_donation(amount);

    // 初始化或更新捐助排行榜
    if ctx.accounts.donation_leaderboard.total_donors == 0 {
        // 首次捐助，初始化排行榜
        ctx.accounts.donation_leaderboard.initialize(
            ctx.bumps.donation_leaderboard,
            ctx.accounts.temple_config.donation_deadline,
        );
    }

    // 更新排行榜
    ctx.accounts
        .donation_leaderboard
        .update_donation(donor.key(), amount);

    // 更新全局统计
    ctx.accounts.global_stats.add_donation(amount);

    // 发出捐助完成事件
    emit!(DonationCompleted {
        user: donor.key(),
        amount,
        total_donated: ctx.accounts.user_donation_state.donation_amount,
        level: ctx.accounts.user_donation_state.donation_level,
        timestamp: clock.unix_timestamp,
    });

    let donation_sol = amount as f64 / 1_000_000_000.0;
    msg!("用户 {} 捐助了 {:.6} SOL", donor.key(), donation_sol);
    msg!(
        "当前捐助等级: {}",
        ctx.accounts.user_donation_state.donation_level
    );

    Ok(())
}
