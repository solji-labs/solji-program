use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

// 寺庙统计数据结构
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TempleStats {
    // 基础统计
    pub total_users: u64,
    pub total_draw_fortune: u64,
    pub total_wishes: u64,
    pub total_donations_sol: f64,

    // NFT统计
    pub total_fortune_nfts: u64,
    pub total_amulets: u64,
    pub total_buddha_nfts: u32,
    pub total_medal_nfts: u32,

    // 活动统计（最近24小时）
    pub recent_draws: u64,
    pub recent_wishes: u64,
    pub recent_donations_lamports: u64,

    // 寺庙状态
    pub current_level: u8,
    pub incense_points: u64,

    // 元数据
    pub last_updated: i64,
}

#[derive(Accounts)]
pub struct GetTempleStats<'info> {
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

pub fn get_temple_stats(ctx: Context<GetTempleStats>) -> Result<TempleStats> {
    let temple_config = &ctx.accounts.temple_config;
    let global_stats = &ctx.accounts.global_stats;

    // TODO: 实现最近24小时的活动统计
    // 目前返回总计数作为占位符
    // 未来需要添加历史数据存储和查询机制
    let recent_draws = global_stats.total_draw_fortune;
    let recent_wishes = global_stats.total_wishes;
    let recent_donations_lamports = global_stats.total_donations_lamports;

    Ok(TempleStats {
        // 基础统计
        total_users: global_stats.total_users,
        total_draw_fortune: global_stats.total_draw_fortune,
        total_wishes: global_stats.total_wishes,
        total_donations_sol: global_stats.total_donations_sol(),

        // NFT统计
        total_fortune_nfts: global_stats.total_fortune_nfts,
        total_amulets: global_stats.total_amulets,
        total_buddha_nfts: temple_config.total_buddha_nft,
        total_medal_nfts: temple_config.total_medal_nft,

        // 活动统计（最近24小时）
        recent_draws,
        recent_wishes,
        recent_donations_lamports,

        // 寺庙状态
        current_level: temple_config.level,
        incense_points: global_stats.total_incense_points,

        // 元数据
        last_updated: global_stats.updated_at,
    })
}
