use crate::error::ErrorCode;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ShareFortune<'info> {
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
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,
}

pub fn share_fortune(ctx: Context<ShareFortune>, share_hash: [u8; 32]) -> Result<()> {
    // Verify user has drawn fortune in the last day
    let now = Clock::get()?.unix_timestamp;
    let time_since_last_draw: i64 = now - ctx.accounts.user_incense_state.last_draw_time;

    require!(
        time_since_last_draw <= 24 * 60 * 60,
        ErrorCode::ShareTooLate
    );

    // Reward
    ctx.accounts
        .user_incense_state
        .add_incense_value_and_merit(0, 1);
    msg!("Fortune sharing successful, earned 0.1 merit points reward");

    Ok(())
}
