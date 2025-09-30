use crate::error::ErrorCode;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

// Temple level information
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TempleLevelInfo {
    pub current_level: u8,                                      // Current level
    pub incense_points: u64,                                    // Current incense points
    pub total_draw_fortune: u64,                                // Total draw fortune count
    pub total_wishes: u64,                                      // Total wish count
    pub total_donations_sol: f64,                               // Total donation amount
    pub total_fortune_nfts: u64,                                // Total fortune NFT count
    pub next_level_requirements: Option<NextLevelRequirements>, // Next level requirements
}

// Next level upgrade requirements
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct NextLevelRequirements {
    pub target_level: u8,             // Target level
    pub required_incense_points: u64, // Required incense points
    pub required_draw_fortune: u64,   // Required draw fortune count
    pub required_wishes: u64,         // Required wish count
    pub required_donations_sol: f64,  // Required donation amount (SOL)
    pub required_fortune_nfts: u64,   // Required fortune NFT count
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

    // Calculate current level
    let current_level = temple_config.calculate_temple_level(global_stats);

    // Find next level requirements from config
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
