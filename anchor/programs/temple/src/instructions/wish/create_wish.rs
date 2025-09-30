use crate::error::ErrorCode;
use crate::state::event::WishCreated;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
use crate::state::wish::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(wish_id: u64)]
pub struct CreateWish<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Wish::INIT_SPACE,
        seeds = [Wish::SEED_PREFIX.as_bytes(), user.key().as_ref(), &wish_id.to_le_bytes()],
        bump
    )]
    pub wish_account: Box<Account<'info, Wish>>,

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
        constraint = user_state.user == user.key() @ ErrorCode::InvalidUserState
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

    #[account(
        mut,
        seeds = [GlobalStats::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub global_stats: Account<'info, GlobalStats>,

    pub system_program: Program<'info, System>,
}

pub fn create_wish(
    ctx: Context<CreateWish>,
    wish_id: u64,
    content_hash: [u8; 32],
    is_anonymous: bool,
) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::CreateWish,
        current_time,
    )?;

    // User state modification
    let user_incense_state = &mut ctx.accounts.user_incense_state;

    // Check daily limit
    if !user_incense_state.can_wish_free() {
        user_incense_state.consume_merit_for_wish(5)?;
    }
    // Update
    user_incense_state.update_wish_count();

    // Update global stats
    ctx.accounts.global_stats.increment_wishes();

    // Give merit reward
    if user_incense_state.can_wish_free() {
        user_incense_state.add_incense_value_and_merit(0, 1);
    }

    // Create wish
    let wish = &mut ctx.accounts.wish_account;
    let user = &ctx.accounts.user;
    let clock = Clock::get()?;
    // Set to account
    wish.id = wish_id;
    wish.creator = user.key();
    wish.content_hash = content_hash;
    wish.is_anonymous = is_anonymous;
    wish.created_at = clock.unix_timestamp;
    wish.likes = 0;
    wish.bump = ctx.bumps.wish_account;

    // Amulet drop probability logic: 10% chance
    let random_seed = (clock.unix_timestamp as u64).wrapping_add(wish_id);
    let amulet_drop_random = (random_seed % 100) as u8;
    let amulet_dropped = amulet_drop_random < 10;
    if amulet_dropped {
        // Increase user's mintable amulet balance
        ctx.accounts.user_state.pending_amulets += 1;
        msg!(
            "Congratulations! Got 1 amulet minting opportunity from making a wish! Current balance: {}",
            ctx.accounts.user_state.pending_amulets
        );
    }

    // Emit wish created event
    emit!(WishCreated {
        user: ctx.accounts.user.key(),
        wish_id,
        is_anonymous,
        amulet_dropped,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
