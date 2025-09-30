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
    pub daily_rank: Option<u32>,   // Daily rank
    pub weekly_rank: Option<u32>,  // Weekly rank
    pub monthly_rank: Option<u32>, // Monthly rank
    pub has_visual_effect: bool,   // Whether has visual effect reward
}

pub fn get_incense_leaderboard(ctx: Context<GetIncenseLeaderboard>) -> Result<IncenseLeaderBoard> {
    let leaderboard = &ctx.accounts.leaderboard;
    let user = &ctx.accounts.user.key();

    // Get daily rank
    let daily_rank = leaderboard.get_incense_leaderboard(user, LeaderboardPeriod::Daily);

    // Get weekly rank
    let weekly_rank = leaderboard.get_incense_leaderboard(user, LeaderboardPeriod::Weekly);

    // Get monthly rank
    let monthly_rank = leaderboard.get_incense_leaderboard(user, LeaderboardPeriod::Monthly);

    // Check if has visual effect reward (top 3 in any period)
    let has_visual_effect = leaderboard.has_visual_effect(user, LeaderboardPeriod::Daily)
        || leaderboard.has_visual_effect(user, LeaderboardPeriod::Weekly)
        || leaderboard.has_visual_effect(user, LeaderboardPeriod::Monthly);

    msg!(
        "User rank query completed - Daily rank: {:?}, Weekly rank: {:?}, Monthly rank: {:?}, Visual effect: {}",
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
