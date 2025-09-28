use crate::error::ErrorCode;
use crate::state::donation_leaderboard::DonationLeaderboard;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitDonationLeaderboard<'info> {
    #[account(
        mut,
        address = crate::admin::ID @ ErrorCode::InvalidOwner
    )]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [DonationLeaderboard::SEED_PREFIX.as_bytes()],
        bump,
        payer = owner,
        space = 8 + DonationLeaderboard::INIT_SPACE
    )]
    pub donation_leaderboard: Box<Account<'info, DonationLeaderboard>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn init_donation_leaderboard(
    ctx: Context<InitDonationLeaderboard>,
    donation_deadline: u64,
) -> Result<()> {
    let donation_leaderboard = &mut ctx.accounts.donation_leaderboard;

    // 验证截止时间不能早于当前时间
    let current_time = Clock::get()?.unix_timestamp as u64;
    require!(donation_deadline > current_time, ErrorCode::InvalidAmount);

    // 初始化排行榜
    donation_leaderboard.initialize(ctx.bumps.donation_leaderboard, donation_deadline);

    msg!(
        "Donation leaderboard initialized with deadline: {}",
        donation_deadline
    );
    Ok(())
}
