use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalStats {
    pub temple_config: Pubkey, // Associated temple config
    // Core statistics
    pub total_incense_points: u64,     // Total incense points
    pub total_merit: u64,              // Total merit
    pub total_draw_fortune: u64,       // Total draw fortune count
    pub total_wishes: u64,             // Total wishes
    pub total_donations_lamports: u64, // Total donation amount
    pub total_users: u64,              // Total users
    // NFT statistics
    pub total_fortune_nfts: u64,  // Total fortune NFTs
    pub total_amulets: u64,       // Total amulets
    pub total_buddha_lights: u64, // Total buddha lights
    // Metadata
    pub updated_at: i64, // Last updated time
}

impl GlobalStats {
    pub const SEED_PREFIX: &str = "global_stats_v1";

    // Convert
    pub fn total_donations_sol(&self) -> f64 {
        self.total_donations_lamports as f64 / 1_000_000_000.0
    }

    // Draw fortune
    pub fn increment_draw_fortune(&mut self) {
        self.total_draw_fortune = self.total_draw_fortune.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // Make wish
    pub fn increment_wishes(&mut self) {
        self.total_wishes = self.total_wishes.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // Wish tower
    pub fn increment_wish_towers(&mut self) {
        // For now, we don't track total wish towers in global stats
        // This method is here for consistency and future expansion
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // Donate
    pub fn add_donation(&mut self, amount_lamports: u64) {
        self.total_donations_lamports = self
            .total_donations_lamports
            .saturating_add(amount_lamports);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // Fortune NFT
    pub fn increment_fortune_nfts(&mut self) {
        self.total_fortune_nfts = self.total_fortune_nfts.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // TODO Amulet
    pub fn increment_amulets(&mut self) {
        self.total_amulets = self.total_amulets.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // Incense points and merit
    pub fn add_incense_value_and_merit(&mut self, incense_value: u64, merit: u64) {
        self.total_incense_points = self.total_incense_points.saturating_add(incense_value);
        self.total_merit = self.total_merit.saturating_add(merit);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // User count
    pub fn increment_users(&mut self) {
        self.total_users = self.total_users.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // TODO When is buddha light
    pub fn increment_buddha_lights(&mut self) {
        self.total_buddha_lights = self.total_buddha_lights.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }
}
