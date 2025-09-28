use crate::error::ErrorCode;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

// 寺庙等级信息
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TempleLevelInfo {
    pub current_level: u8,                                      // 当前等级
    pub incense_points: u64,                                    // 当前香火值
    pub total_draw_fortune: u64,                                // 总抽签次数
    pub total_wishes: u64,                                      // 总许愿次数
    pub total_donations_sol: f64,                               // 总捐助金额
    pub total_fortune_nfts: u64,                                // 总签文NFT数量
    pub next_level_requirements: Option<NextLevelRequirements>, // 下一等级要求
}

// 下一等级升级要求
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NextLevelRequirements {
    pub target_level: u8,             // 目标等级
    pub required_incense_points: u64, // 需要香火值
    pub required_draw_fortune: u64,   // 需要抽签次数
    pub required_wishes: u64,         // 需要许愿次数
    pub required_donations_sol: f64,  // 需要捐助金额 (SOL)
    pub required_fortune_nfts: u64,   // 需要签文NFT数量
}

#[derive(Accounts)]
pub struct GetTempleLevel<'info> {
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

pub fn get_temple_level(ctx: Context<GetTempleLevel>) -> Result<TempleLevelInfo> {
    let temple_config = &ctx.accounts.temple_config;
    let global_stats = &ctx.accounts.global_stats;

    // 计算当前等级
    let current_level = temple_config.calculate_temple_level(global_stats);

    // 从配置中查找下一等级要求
    let next_level_requirements = temple_config
        .dynamic_config
        .temple_levels
        .iter()
        .find(|level_config| level_config.level == current_level + 1)
        .map(|level_config| NextLevelRequirements {
            target_level: level_config.level,
            required_incense_points: level_config.required_incense_points,
            required_draw_fortune: level_config.required_draw_fortune,
            required_wishes: level_config.required_wishes,
            required_donations_sol: level_config.required_donations_sol,
            required_fortune_nfts: level_config.required_fortune_nfts,
        });

    Ok(TempleLevelInfo {
        current_level,
        incense_points: global_stats.total_incense_points,
        total_draw_fortune: global_stats.total_draw_fortune,
        total_wishes: global_stats.total_wishes,
        total_donations_sol: global_stats.total_donations_sol(),
        total_fortune_nfts: global_stats.total_fortune_nfts,
        next_level_requirements,
    })
}
