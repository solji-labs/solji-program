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
    pub const WISHES_PER_LEVEL: u32 = 3; //TODO

    // Calculate level based on wish count
    pub fn calculate_level(wish_count: u32) -> u8 {
        (wish_count / Self::WISHES_PER_LEVEL) as u8 + 1
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
