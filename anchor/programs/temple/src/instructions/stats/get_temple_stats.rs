use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

// Temple statistics data structure
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct TempleStats {
    // Basic statistics
    pub total_users: u64,
    pub total_draw_fortune: u64,
    pub total_wishes: u64,
    pub total_donations_sol: f64,

    // NFT statistics
    pub total_fortune_nfts: u64,
    pub total_amulets: u64,
    pub total_buddha_nfts: u32,
    pub total_medal_nfts: u32,

    // Activity statistics (last 24 hours)
    pub recent_draws: u64,
    pub recent_wishes: u64,
    pub recent_donations_lamports: u64,

    // Temple status
    pub current_level: u8,
    pub incense_points: u64,

    // Metadata
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

    // TODO: Implement activity statistics for the last 24 hours
    // Currently returns total count as placeholder
    // Future needs to add historical data storage and query mechanism
    let recent_draws = global_stats.total_draw_fortune;
    let recent_wishes = global_stats.total_wishes;
    let recent_donations_lamports = global_stats.total_donations_lamports;

    Ok(TempleStats {
        // Basic statistics
        total_users: global_stats.total_users,
        total_draw_fortune: global_stats.total_draw_fortune,
        total_wishes: global_stats.total_wishes,
        total_donations_sol: global_stats.total_donations_sol(),

        // NFT statistics
        total_fortune_nfts: global_stats.total_fortune_nfts,
        total_amulets: global_stats.total_amulets,
        total_buddha_nfts: temple_config.total_buddha_nft,
        total_medal_nfts: temple_config.total_medal_nft,

        // Activity statistics (last 24 hours)
        recent_draws,
        recent_wishes,
        recent_donations_lamports,

        // Temple status
        current_level: temple_config.level,
        incense_points: global_stats.total_incense_points,

        // Metadata
        last_updated: global_stats.updated_at,
    })
}
