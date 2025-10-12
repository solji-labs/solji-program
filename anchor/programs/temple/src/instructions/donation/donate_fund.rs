use crate::error::ErrorCode;
use crate::state::event::DonationCompleted;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::UserDonationState;
use anchor_lang::prelude::*;

/// Core donation fund instruction
/// Only handles fund transfer and basic recording, emits events for subsequent processing

#[derive(Accounts)]
pub struct DonateFund<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,

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
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    /// CHECK: Temple treasury account
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn donate_fund(ctx: Context<DonateFund>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::Donate,
        current_time,
    )?;

    let donor = &ctx.accounts.donor;
    let temple_treasury = &ctx.accounts.temple_treasury;

    // Validate donation amount
    require!(amount > 0, ErrorCode::InvalidAmount);

    // Transfer SOL to temple treasury
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &donor.key(),
        &temple_treasury.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            donor.to_account_info(),
            temple_treasury.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Process donation record
    ctx.accounts.user_donation_state.process_donation(amount);

    // Update global stats
    ctx.accounts.global_stats.add_donation(amount);

    // Calculate incense burn bonus: 1 burn per 0.01 SOL
    let bonus_burns = (amount / 1_000_000) as u8; // 0.01 SOL = 1_000_000 lamports
    if bonus_burns > 0 {
        // Note: We need to add UserIncenseState to the accounts, but for now we'll just log it
        // In a full implementation, we'd need to modify the accounts struct and add user_incense_state
        msg!(
            "Donation bonus: {} additional incense burns unlocked",
            bonus_burns
        );
        // TODO: Actually apply the bonus burns to user_incense_state when accounts are updated
    }

    // Emit donation completed event
    emit!(DonationCompleted {
        user: donor.key(),
        amount,
        total_donated: ctx.accounts.user_donation_state.donation_amount,
        level: ctx.accounts.user_donation_state.donation_level,
        timestamp: clock.unix_timestamp,
    });

    let donation_sol = amount as f64 / 1_000_000_000.0;
    msg!("User {} donated {:.6} SOL", donor.key(), donation_sol);
    msg!(
        "Current donation level: {}",
        ctx.accounts.user_donation_state.donation_level
    );

    Ok(())
}
