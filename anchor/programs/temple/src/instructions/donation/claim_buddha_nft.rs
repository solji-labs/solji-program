use crate::error::ErrorCode;
use crate::state::donation_leaderboard::DonationLeaderboard;
use crate::state::temple_config::*;
use crate::state::user_state::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimBuddhaNft<'info> {
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
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        seeds = [DonationLeaderboard::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub donation_leaderboard: Box<Account<'info, DonationLeaderboard>>,
}

pub fn claim_buddha_nft(ctx: Context<ClaimBuddhaNft>) -> Result<()> {
    let user = &ctx.accounts.user;

    // Check if distribution is completed
    require!(
        ctx.accounts.donation_leaderboard.distribution_completed,
        ErrorCode::NotApproved
    );

    // Check if user already has Buddha NFT
    require!(
        !ctx.accounts.user_state.has_buddha_nft,
        ErrorCode::UserHasBuddhaNFT
    );

    // Check if user is among the top 10,000 donors
    require!(
        ctx.accounts.donation_leaderboard.is_top_donor(&user.key()),
        ErrorCode::InsufficientDonation
    );

    // Set user to have Buddha NFT eligibility
    ctx.accounts.user_state.has_buddha_nft = true;

    msg!(
        "User {} successfully claimed Buddha NFT eligibility",
        user.key()
    );

    Ok(())
}
