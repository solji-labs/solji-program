use crate::state::leaderboard::{Leaderboard, LeaderboardPeriod};
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;

// TODO Update incense leaderboard whenever incense-related events are detected
#[derive(Accounts)]
#[instruction(period: LeaderboardPeriod)]
pub struct UpdateLeaderboard<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump = user_state.bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump = user_incense_state.bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        mut,
        seeds = [Leaderboard::SEED_PREFIX.as_bytes()],
        bump = leaderboard.bump,
    )]
    pub leaderboard: Box<Account<'info, Leaderboard>>,

    pub system_program: Program<'info, System>,
}

pub fn update_leaderboard(
    ctx: Context<UpdateLeaderboard>,
    period: LeaderboardPeriod,
) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let user_incense_state = &ctx.accounts.user_incense_state;
    let user = ctx.accounts.user.key();

    // Check and reset expired periods
    leaderboard.check_and_reset_periods();

    // Select value based on period
    let value = match period {
        LeaderboardPeriod::Daily => user_incense_state.incense_number as u64,
        LeaderboardPeriod::Weekly => user_incense_state.incense_points,
        LeaderboardPeriod::Monthly => user_incense_state.merit,
    };

    // Update leaderboard ranking
    leaderboard.update_user_ranking(user, value, period.clone());

    msg!(
        "排行榜更新完成 - 用户: {}, 周期: {:?}, 值: {}",
        user,
        period,
        value
    );

    Ok(())
}
