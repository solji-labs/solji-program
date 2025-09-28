use crate::error::ErrorCode;
use crate::state::donation_leaderboard::DonationLeaderboard;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;
/// 分配Buddha NFT给前10,000名捐助者
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

    // 检查是否可以开始分配
    require!(
        ctx.accounts
            .donation_leaderboard
            .can_start_distribution(current_time),
        ErrorCode::NotApproved
    );

    // 获取前10,000名捐助者数量
    let top_donors_count = ctx
        .accounts
        .donation_leaderboard
        .get_top_donors(10000)
        .len();

    if top_donors_count == 0 {
        msg!("没有符合条件的捐助者");
        return Ok(());
    }

    // 标记分配完成
    ctx.accounts
        .donation_leaderboard
        .mark_distribution_completed(top_donors_count as u32);

    msg!(
        "Buddha NFT分配完成，为前 {} 名捐助者设置了资格",
        top_donors_count
    );

    Ok(())
}
