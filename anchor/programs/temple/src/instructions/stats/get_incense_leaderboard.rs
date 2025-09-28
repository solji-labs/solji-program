use crate::state::leaderboard::{Leaderboard, LeaderboardPeriod};
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct GetIncenseLeaderboard<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [Leaderboard::SEED_PREFIX.as_bytes()],
        bump = leaderboard.bump,
    )]
    pub leaderboard: Box<Account<'info, Leaderboard>>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct IncenseLeaderBoard {
    pub daily_rank: Option<u32>,   // 每日排名
    pub weekly_rank: Option<u32>,  // 每周排名
    pub monthly_rank: Option<u32>, // 每月排名
    pub has_visual_effect: bool,   // 是否有视觉特效奖励
}

pub fn get_incense_leaderboard(ctx: Context<GetIncenseLeaderboard>) -> Result<IncenseLeaderBoard> {
    let leaderboard = &ctx.accounts.leaderboard;
    let user = &ctx.accounts.user.key();

    // 获取每日排名
    let daily_rank = leaderboard.get_incense_leaderboard(user, LeaderboardPeriod::Daily);

    // 获取每周排名
    let weekly_rank = leaderboard.get_incense_leaderboard(user, LeaderboardPeriod::Weekly);

    // 获取每月排名
    let monthly_rank = leaderboard.get_incense_leaderboard(user, LeaderboardPeriod::Monthly);

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

    Ok(IncenseLeaderBoard {
        daily_rank,
        weekly_rank,
        monthly_rank,
        has_visual_effect,
    })
}
