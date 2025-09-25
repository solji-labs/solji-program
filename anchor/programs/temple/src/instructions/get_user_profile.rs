use crate::error::ErrorCode;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserDonationState, UserIncenseState, UserState, UserTitle};
use anchor_lang::prelude::*;

// 用户概览信息
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserProfile {
    // 基本信息
    pub user: Pubkey,
    pub title: UserTitle,
    pub has_buddha_nft: bool,
    pub has_medal_nft: bool,

    // 香火相关
    pub incense_points: u64, // 香火值
    pub merit: u64,          // 功德值
    pub incense_number: u8,  // 每日烧香量

    // 捐助相关
    pub donation_amount: u64, // 捐助金额 (lamports)
    pub donation_level: u8,   // 捐助等级

    // NFT持有情况
    pub total_buddha_nfts: u32, // 佛像NFT数量
    pub total_medal_nfts: u32,  // 勋章NFT数量
    pub total_amulets: u32,     // 御守数量

    // 活动统计
    pub total_draws: u32,     // 总抽签次数
    pub total_wishes: u32,    // 总许愿次数
    pub total_donations: u32, // 总捐助次数

    // 时间信息
    pub join_time: i64,     // 加入时间
    pub last_activity: i64, // 最后活动时间
}

#[derive(Accounts)]
pub struct GetUserProfile<'info> {
    pub user: Signer<'info>,

    #[account(
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_incense_state: Account<'info, UserIncenseState>,

    #[account(
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_donation_state: Account<'info, UserDonationState>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump
    )]
    pub temple_config: Account<'info, TempleConfig>,

    #[account(
        seeds = [GlobalStats::SEED_PREFIX.as_bytes()],
        bump
    )]
    pub global_stats: Account<'info, GlobalStats>,
}

pub fn get_user_profile(ctx: Context<GetUserProfile>) -> Result<UserProfile> {
    let user_state = &ctx.accounts.user_state;
    let user_incense_state = &ctx.accounts.user_incense_state;
    let user_donation_state = &ctx.accounts.user_donation_state;

    // 计算加入时间 (使用用户状态创建时间，这里简化为当前时间)
    let join_time = Clock::get()?.unix_timestamp;

    // 计算最后活动时间 (取各个状态的最新更新时间)
    let last_activity = user_incense_state
        .update_time
        .max(user_donation_state.last_donation_time);

    Ok(UserProfile {
        user: *ctx.accounts.user.key,
        title: user_incense_state.title.clone(),
        has_buddha_nft: user_state.has_buddha_nft,
        has_medal_nft: user_state.has_medal_nft,

        incense_points: user_incense_state.incense_points,
        merit: user_incense_state.merit,
        incense_number: user_incense_state.incense_number,

        donation_amount: user_donation_state.donation_amount,
        donation_level: user_donation_state.donation_level,

        // 用户个人统计信息
        total_buddha_nfts: if user_state.has_buddha_nft { 1 } else { 0 },
        total_medal_nfts: if user_state.has_medal_nft { 1 } else { 0 },
        total_amulets: user_state.pending_amulets,

        total_draws: user_incense_state.total_draws,
        total_wishes: user_incense_state.total_wishes,
        total_donations: user_donation_state.total_donation_count,

        join_time,
        last_activity,
    })
}
