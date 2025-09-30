use crate::error::ErrorCode;
use crate::state::donation_leaderboard::DonationLeaderboard;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;
/// Distribute Buddha NFTs to the first 10,000 donors

#[derive(Accounts)]
pub struct DistributeBuddhaNfts<'info> {
    #[account(
        mut,
        address = crate::admin::ID @ ErrorCode::InvalidOwner
    )]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        mut,
        seeds = [DonationLeaderboard::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub donation_leaderboard: Box<Account<'info, DonationLeaderboard>>,
}

pub fn distribute_buddha_nfts(ctx: Context<DistributeBuddhaNfts>) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    require!(
        ctx.accounts
            .donation_leaderboard
            .can_start_distribution(current_time),
        ErrorCode::NotApproved
    );

    let top_donors_count = ctx
        .accounts
        .donation_leaderboard
        .get_top_donors(10000)
        .len();

    if top_donors_count == 0 {
        msg!("No eligible donors");
        return Ok(());
    }

    ctx.accounts
        .donation_leaderboard
        .mark_distribution_completed(top_donors_count as u32);

    msg!(
        "Buddha NFT distribution completed, qualification set for the first {} donors",
        top_donors_count
    );

    Ok(())
}
