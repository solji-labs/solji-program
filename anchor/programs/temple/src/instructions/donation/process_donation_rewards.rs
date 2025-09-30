use crate::error::ErrorCode;
use crate::state::event::RewardsProcessed;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserDonationState, UserIncenseState};
use anchor_lang::prelude::*;

/// Process donation rewards instruction
/// Listen to donation completed events, handle level rewards and dynamic configuration rewards
#[derive(Accounts)]
pub struct ProcessDonationRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

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
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,
}

pub fn process_donation_rewards(ctx: Context<ProcessDonationRewards>) -> Result<()> {
    let clock = Clock::get()?;

    // Get donation level rewards
    let (merit_reward, incense_points_reward) =
        ctx.accounts.user_donation_state.get_donation_rewards();

    // Update user incense state
    if merit_reward > 0 || incense_points_reward > 0 {
        ctx.accounts
            .user_incense_state
            .add_incense_value_and_merit(incense_points_reward, merit_reward);

        msg!(
            "Donation reward earned - Merit: {}, Incense points: {}",
            merit_reward,
            incense_points_reward
        );
    }

    // Update global stats
    ctx.accounts
        .global_stats
        .add_incense_value_and_merit(incense_points_reward, merit_reward);

    // Process donation unlock incense logic - read from dynamic config
    let total_donation_sol =
        ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0;

    // Iterate through all donation reward configurations
    for reward_config in &ctx.accounts.temple_config.dynamic_config.donation_rewards {
        // Check if minimum donation amount threshold is reached
        if total_donation_sol >= reward_config.min_donation_sol {
            // Calculate the total reward that should be obtained currently
            let current_reward = if reward_config.burn_bonus_per_001_sol > 0 {
                // Burn incense bonus: increase burn count per 0.01 SOL
                ((total_donation_sol * 100.0) as u64)
                    .saturating_mul(reward_config.burn_bonus_per_001_sol)
            } else {
                // Incense reward: cumulative reward based on threshold
                let current_tier = (total_donation_sol / reward_config.min_donation_sol) as u64;
                current_tier.saturating_mul(reward_config.incense_amount)
            };

            // Here we can record processed rewards to avoid duplicate distribution
            // Actual implementation requires additional state tracking

            if current_reward > 0 {
                if reward_config.burn_bonus_per_001_sol > 0 {
                    // Burn incense bonus
                    ctx.accounts.user_incense_state.incense_number = ctx
                        .accounts
                        .user_incense_state
                        .incense_number
                        .saturating_add(current_reward as u8);
                    msg!(
                        "Donation earned extra burn incense count: {}",
                        current_reward
                    );
                } else {
                    // Incense reward
                    ctx.accounts
                        .user_incense_state
                        .add_incense_balance(reward_config.incense_id, current_reward);
                    msg!(
                        "Donation unlocked incense type {}: {} sticks",
                        reward_config.incense_id,
                        current_reward
                    );
                }
            }
        }
    }

    // Emit rewards processed event
    emit!(RewardsProcessed {
        user: ctx.accounts.user.key(),
        merit_reward,
        incense_points_reward,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
