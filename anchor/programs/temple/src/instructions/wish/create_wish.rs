use crate::error::ErrorCode;
use crate::state::event::{WishCreated, WishTowerUpdated};
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
use crate::state::wish::*;
use crate::state::wish_tower::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateWish<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + Wish::INIT_SPACE,
        seeds = [Wish::SEED_PREFIX.as_bytes(), user.key().as_ref(), (user_incense_state.total_wishes + 1).to_string().as_ref()],
        bump
    )]
    pub wish_account: Box<Account<'info, Wish>>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8 + WishTower::INIT_SPACE,
        seeds = [WishTower::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub wish_tower_account: Box<Account<'info, WishTower>>,

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

    // Calculate new wish id from total_wishes + 1
    let new_wish_id = (user_incense_state.total_wishes + 1) as u64;

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
    wish.id = new_wish_id;
    wish.creator = user.key();
    wish.content_hash = content_hash;
    wish.is_anonymous = is_anonymous;
    wish.created_at = clock.unix_timestamp;
    wish.likes = 0;
    wish.bump = ctx.bumps.wish_account;

    // Amulet drop probability logic: 10% chance
    #[cfg(feature = "localnet")]
    let amulet_dropped = true; // Test environment: 100% drop rate for amulet

    #[cfg(not(feature = "localnet"))]
    let amulet_dropped = {
        let random_seed = (clock.unix_timestamp as u64).wrapping_add(new_wish_id);
        let amulet_drop_random = (random_seed % 100) as u8;
        amulet_drop_random < 10
    };
    if amulet_dropped {
        // Emit amulet dropped event with type information
        msg!("Congratulations! Got 1 Protection Amulet NFT from making a wish!");
        emit!(crate::state::event::AmuletDropped {
            user: ctx.accounts.user.key(),
            amulet_type: 1, // Protection Amulet
            source: "create_wish".to_string(),
            timestamp: clock.unix_timestamp,
        });
    }

    // Update wish tower
    let wish_tower = &mut ctx.accounts.wish_tower_account;
    let is_new_tower = wish_tower.creator == Pubkey::default();

    if is_new_tower {
        // Initialize new tower
        wish_tower.creator = ctx.accounts.user.key();
        wish_tower.wish_count = 0;
        wish_tower.level = 0;
        wish_tower.wish_ids = Vec::new();
        wish_tower.created_at = clock.unix_timestamp;
        wish_tower.last_updated = clock.unix_timestamp;
        wish_tower.bump = ctx.bumps.wish_tower_account;
    }

    // Add wish to tower
    wish_tower.add_wish(new_wish_id);

    // Emit wish created event
    emit!(WishCreated {
        user: ctx.accounts.user.key(),
        wish_id: new_wish_id,
        content_hash,
        is_anonymous,
        amulet_dropped,
        timestamp: clock.unix_timestamp,
    });

    // Emit wish tower updated event
    emit!(WishTowerUpdated {
        user: ctx.accounts.user.key(),
        wish_count: wish_tower.wish_count,
        level: wish_tower.level,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
