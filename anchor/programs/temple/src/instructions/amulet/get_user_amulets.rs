use crate::error::ErrorCode;
use crate::state::user_state::UserState;
use anchor_lang::prelude::*;

// User Amulets Collection Info Panel
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserAmuletsInfo {
    pub user: Pubkey,
    // Amulet count statistics
    pub total_amulets: u32,
    // Amulet counts by source
    pub draw_fortune_count: u32,
    pub make_wish_count: u32,
    // Mintable amulet balance
    pub pending_amulets: u32, // Number of mintable amulets
    // Last updated time
    pub last_updated: i64,
}

#[derive(Accounts)]
pub struct GetUserAmulets<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
}

pub fn get_user_amulets(ctx: Context<GetUserAmulets>) -> Result<UserAmuletsInfo> {
    let user = *ctx.accounts.user.key;

    // TODO Get amulet info directly from user state
    Ok(UserAmuletsInfo {
        user,
        total_amulets: 0,
        draw_fortune_count: 0,
        make_wish_count: 0,
        pending_amulets: ctx.accounts.user_state.pending_amulets,
        last_updated: Clock::get()?.unix_timestamp,
    })
}
