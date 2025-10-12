use crate::state::global_stats::GlobalStats;
use crate::state::user_state::{
    DailyIncenseCount, IncenseBalance, UserDonationState, UserIncenseState, UserState,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserState::INIT_SPACE,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        init,
        payer = user,
        space = 8 + UserIncenseState::INIT_SPACE,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        init,
        payer = user,
        space = 8 + UserDonationState::INIT_SPACE,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    #[account(
        mut,
        seeds = [GlobalStats::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub global_stats: Account<'info, GlobalStats>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    let user_incense_state = &mut ctx.accounts.user_incense_state;
    let user_donation_state = &mut ctx.accounts.user_donation_state;
    let user = &ctx.accounts.user;
    let clock = Clock::get()?;

    // Initialize main user state
    user_state.user = user.key();
    user_state.has_buddha_nft = false;
    user_state.has_medal_nft = false;
    user_state.bump = ctx.bumps.user_state;

    // Initialize incense state
    user_incense_state.user = user.key();
    user_incense_state.title = crate::state::user_state::UserTitle::Pilgrim;
    user_incense_state.incense_points = 0;
    user_incense_state.merit = 0;
    user_incense_state.incense_number = 0;
    user_incense_state.update_time = clock.unix_timestamp;
    user_incense_state.bump = ctx.bumps.user_incense_state;

    // Initialize incense balance and daily count arrays
    user_incense_state.incense_balance = [
        IncenseBalance {
            incense_id: 0,
            balance: 0,
        },
        IncenseBalance {
            incense_id: 0,
            balance: 0,
        },
        IncenseBalance {
            incense_id: 0,
            balance: 0,
        },
        IncenseBalance {
            incense_id: 0,
            balance: 0,
        },
        IncenseBalance {
            incense_id: 0,
            balance: 0,
        },
        IncenseBalance {
            incense_id: 0,
            balance: 0,
        },
    ];
    user_incense_state.daily_incense_count = [
        DailyIncenseCount {
            incense_id: 0,
            count: 0,
        },
        DailyIncenseCount {
            incense_id: 0,
            count: 0,
        },
        DailyIncenseCount {
            incense_id: 0,
            count: 0,
        },
        DailyIncenseCount {
            incense_id: 0,
            count: 0,
        },
        DailyIncenseCount {
            incense_id: 0,
            count: 0,
        },
        DailyIncenseCount {
            incense_id: 0,
            count: 0,
        },
    ];

    // Initialize draw fortune and wish related
    user_incense_state.daily_draw_count = 0;
    user_incense_state.last_draw_time = 0;
    user_incense_state.total_draws = 0;
    user_incense_state.daily_wish_count = 0;
    user_incense_state.last_wish_time = 0;
    user_incense_state.total_wishes = 0;

    // Initialize donation state
    user_donation_state.user = user.key();
    user_donation_state.donation_amount = 0;
    user_donation_state.donation_level = 0;
    user_donation_state.total_donation_count = 0;
    user_donation_state.last_donation_time = 0;
    user_donation_state.bump = ctx.bumps.user_donation_state;

    // Update global stats user count
    ctx.accounts.global_stats.increment_users();

    msg!("User state initialized for: {}", user.key());
    Ok(())
}
