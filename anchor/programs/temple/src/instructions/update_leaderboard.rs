use crate::state::leaderboard::{Leaderboard, LeaderboardPeriod};
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;

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

    // 检查并重置过期周期
    leaderboard.check_and_reset_periods();

    // 根据周期选择值
    let value = match period {
        LeaderboardPeriod::Daily => user_incense_state.incense_number as u64,
        LeaderboardPeriod::Weekly => user_incense_state.incense_points,
        LeaderboardPeriod::Monthly => user_incense_state.merit,
    };

    // 更新排行榜排名
    leaderboard.update_user_ranking(user, value, period.clone());

    msg!(
        "排行榜更新完成 - 用户: {}, 周期: {:?}, 值: {}",
        user,
        period,
        value
    );

    Ok(())
}

#[derive(Accounts)]
pub struct GetUserRank<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [Leaderboard::SEED_PREFIX.as_bytes()],
        bump = leaderboard.bump,
    )]
    pub leaderboard: Box<Account<'info, Leaderboard>>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserRankResult {
    pub daily_rank: Option<u32>,   // 每日排名
    pub weekly_rank: Option<u32>,  // 每周排名
    pub monthly_rank: Option<u32>, // 每月排名
    pub has_visual_effect: bool,   // 是否有视觉特效奖励
}

pub fn get_user_rank(ctx: Context<GetUserRank>) -> Result<UserRankResult> {
    let leaderboard = &ctx.accounts.leaderboard;
    let user = &ctx.accounts.user.key();

    // 获取每日排名
    let daily_rank = leaderboard.get_user_rank(user, LeaderboardPeriod::Daily);

    // 获取每周排名
    let weekly_rank = leaderboard.get_user_rank(user, LeaderboardPeriod::Weekly);

    // 获取每月排名
    let monthly_rank = leaderboard.get_user_rank(user, LeaderboardPeriod::Monthly);

    // 检查是否有视觉特效奖励（任意周期前3名）
    let has_visual_effect = leaderboard.has_visual_effect(user, LeaderboardPeriod::Daily)
        || leaderboard.has_visual_effect(user, LeaderboardPeriod::Weekly)
        || leaderboard.has_visual_effect(user, LeaderboardPeriod::Monthly);

    msg!(
        "用户排名查询完成 - 每日排名: {:?}, 每周排名: {:?}, 每月排名: {:?}, 视觉特效: {}",
        daily_rank,
        weekly_rank,
        monthly_rank,
        has_visual_effect
    );

    Ok(UserRankResult {
        daily_rank,
        weekly_rank,
        monthly_rank,
        has_visual_effect,
    })
}
