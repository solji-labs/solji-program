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

    // 检查分配是否已完成
    require!(
        ctx.accounts.donation_leaderboard.distribution_completed,
        ErrorCode::NotApproved
    );

    // 检查用户是否已经有Buddha NFT
    require!(
        !ctx.accounts.user_state.has_buddha_nft,
        ErrorCode::UserHasBuddhaNFT
    );

    // 检查用户是否在前10,000名捐助者中
    require!(
        ctx.accounts.donation_leaderboard.is_top_donor(&user.key()),
        ErrorCode::InsufficientDonation
    );

    // 设置用户拥有Buddha NFT资格
    ctx.accounts.user_state.has_buddha_nft = true;

    msg!("用户 {} 成功领取了 Buddha NFT 资格", user.key());

    Ok(())
}
