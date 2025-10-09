use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct WishTower {
    pub creator: Pubkey, // Creator
    pub wish_count: u32, // Total wishes in tower
    pub level: u8,       // Current level (calculated as wish_count / 10)
    #[max_len(1000)] // Allow up to 1000 wishes
    pub wish_ids: Vec<u64>, // Wish IDs in the tower
    pub created_at: i64, // Created timestamp
    pub last_updated: i64, // Last updated timestamp
    pub bump: u8,
}

impl WishTower {
    pub const SEED_PREFIX: &'static str = "wish_tower";

    // Calculate level based on new requirements:
    // Level 0: 0 wishes (obtained by minting)
    // Level 1: 10 wishes
    // Level 2: 50 wishes
    // Level 3: 200 wishes
    // Level 4: 500 wishes
    pub fn calculate_level(wish_count: u32) -> u8 {
        if wish_count >= 500 {
            4 // Perfection Tower
        } else if wish_count >= 200 {
            3 // Grand Wish Tower
        } else if wish_count >= 50 {
            2 // Advanced Tower
        } else if wish_count >= 10 {
            1 // Basic Tower
        } else {
            0 // Seed Tower
        }
    }

    // Get current level
    pub fn get_current_level(&self) -> u8 {
        Self::calculate_level(self.wish_count)
    }

    // Check if can add wish (no limit)
    pub fn can_add_wish(&self) -> bool {
        true // Always can add wishes
    }

    // Add wish and update level
    pub fn add_wish(&mut self, wish_id: u64) {
        self.wish_ids.push(wish_id);
        self.wish_count += 1;
        self.level = self.get_current_level();
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }
}
