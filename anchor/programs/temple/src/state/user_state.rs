use crate::error::ErrorCode;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

// User title enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum UserTitle {
    Pilgrim,   // Pilgrim
    Disciple,  // Disciple
    Protector, // Protector
    Patron,    // Patron
    Abbot,     // Abbot
}

/// Calculate user title based on merit
fn calculate_title_from_merit(merit: u64) -> UserTitle {
    if merit >= 100000 {
        UserTitle::Abbot // Abbot
    } else if merit >= 10000 {
        UserTitle::Patron // Patron
    } else if merit >= 1000 {
        UserTitle::Protector // Protector
    } else if merit >= 100 {
        UserTitle::Disciple // Disciple
    } else {
        UserTitle::Pilgrim // Pilgrim
    }
}

// Define incense balance structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct IncenseBalance {
    pub incense_id: u8,
    pub balance: u64,
}

// Define daily incense burn count structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DailyIncenseCount {
    pub incense_id: u8,
    pub count: u8,
}

// ==== Account structure definition ends ============

// Main user state account
#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,         // User address
    pub has_buddha_nft: bool, // Whether owns Buddha NFT
    pub has_medal_nft: bool,  // Whether owns medal NFT
    pub bump: u8,

    // Random request related
    pub pending_random_request_id: Option<[u8; 32]>, // Pending random request ID

    // Amulet related
    pub pending_amulets: u32, // Amulet balance: for minting
    pub total_amulets: u32,
}

impl UserState {
    pub const SEED_PREFIX: &str = "user_state";
}

// ===== Split sub-accounts =====

// Incense
#[account]
#[derive(InitSpace)]
pub struct UserIncenseState {
    pub user: Pubkey,
    pub title: UserTitle,    // User title (based on merit)
    pub incense_points: u64, // Incense points
    pub merit: u64,          // Merit
    pub incense_number: u8,  // Daily incense burn amount
    pub update_time: i64,    // Update time
    pub bump: u8,

    // Incense balance and daily count
    pub incense_balance: [IncenseBalance; 6],
    pub daily_incense_count: [DailyIncenseCount; 6],

    // Draw fortune related
    pub daily_draw_count: u8,
    pub last_draw_time: i64,
    pub total_draws: u32, // Total draw fortune count

    // Wish related
    pub daily_wish_count: u8,
    pub last_wish_time: i64,
    pub total_wishes: u32, // Total wishes
}

// Donation
#[account]
#[derive(InitSpace)]
pub struct UserDonationState {
    pub user: Pubkey,
    pub donation_amount: u64,
    pub donation_level: u8,
    pub total_donation_count: u32,
    pub last_donation_time: i64,
    pub bump: u8,
}

impl UserIncenseState {
    pub const SEED_PREFIX: &str = "user_incense";

    /// Get balance of specified incense type
    pub fn get_incense_balance(&self, incense_id: u8) -> u64 {
        self.incense_balance
            .iter()
            .find(|item| item.incense_id == incense_id)
            .map(|item| item.balance)
            .unwrap_or(0)
    }

    /// Set balance of specified incense type
    pub fn set_incense_balance(&mut self, incense_id: u8, balance: u64) {
        if let Some(item) = self
            .incense_balance
            .iter_mut()
            .find(|item| item.incense_id == incense_id)
        {
            item.balance = balance;
        } else {
            // Find empty position or replace first empty record
            for item in self.incense_balance.iter_mut() {
                if item.incense_id == 0 || item.incense_id == incense_id {
                    item.incense_id = incense_id;
                    item.balance = balance;
                    break;
                }
            }
        }
    }

    /// Add balance of specified incense type
    pub fn add_incense_balance(&mut self, incense_id: u8, amount: u64) {
        let current_balance = self.get_incense_balance(incense_id);
        self.set_incense_balance(incense_id, current_balance.saturating_add(amount));
    }

    /// Subtract balance of specified incense type
    pub fn subtract_incense_balance(&mut self, incense_id: u8, amount: u64) -> Result<()> {
        let current_balance: u64 = self.get_incense_balance(incense_id);
        if current_balance < amount {
            return err!(ErrorCode::InsufficientIncenseBalance);
        }
        self.set_incense_balance(incense_id, current_balance - amount);
        Ok(())
    }

    /// Get daily incense burn count of specified incense type
    pub fn get_daily_incense_count(&self, incense_id: u8) -> u8 {
        self.daily_incense_count
            .iter()
            .find(|item| item.incense_id == incense_id)
            .map(|item| item.count)
            .unwrap_or(0)
    }

    /// Set daily incense burn count of specified incense type
    pub fn set_daily_incense_count(&mut self, incense_id: u8, count: u8) {
        if let Some(item) = self
            .daily_incense_count
            .iter_mut()
            .find(|item| item.incense_id == incense_id)
        {
            item.count = count;
        } else {
            // Find empty position or replace first empty record
            for item in self.daily_incense_count.iter_mut() {
                if item.incense_id == 0 || item.incense_id == incense_id {
                    item.incense_id = incense_id;
                    item.count = count;
                    break;
                }
            }
        }
    }

    /// Check if incense amount exceeds limit
    pub fn check_daily_incense_limit(&self, incense_id: u8, amount: u8) -> Result<()> {
        // 1. First check if day has changed, reset count if day changed
        let now = Clock::get()?.unix_timestamp;
        let is_new_day = now - self.update_time >= 86400;
        let current_count = if is_new_day {
            0
        } else {
            self.get_daily_incense_count(incense_id)
        };

        // 2. Validate count (single incense type daily ≤10)
        if current_count + amount > 10 {
            return err!(ErrorCode::ExceedDailyIncenseLimit);
        }
        Ok(())
    }

    /// Update daily incense burn count
    pub fn update_daily_count(&mut self, incense_id: u8, amount: u8) {
        let now = Clock::get().unwrap().unix_timestamp;
        // Reset all counts if day changed + update reset time
        let is_new_day = now - self.update_time >= 86400;
        if is_new_day {
            // Manually reset fixed size array
            for item in self.daily_incense_count.iter_mut() {
                item.incense_id = 0;
                item.count = 0;
            }
            self.incense_number = 0;
            self.update_time = now;
        }
        // Accumulate current incense type count
        self.incense_number += amount;
        let current_count = self.get_daily_incense_count(incense_id);
        self.set_daily_incense_count(incense_id, current_count + amount);
    }

    // Add user's incense points and merit, and automatically update title
    pub fn add_incense_value_and_merit(&mut self, incense_value: u64, merit: u64) {
        self.incense_points = self
            .incense_points
            .checked_add(incense_value)
            .unwrap_or(self.incense_points);
        self.merit = self.merit.checked_add(merit).unwrap_or(self.merit);

        // Automatically update title
        self.title = calculate_title_from_merit(self.merit);
    }

    /// Check if can draw fortune for free
    pub fn can_draw_free(&self) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_draw_time >= 86400; // 24 hours

        if is_new_day {
            // New day, reset count
            true
        } else {
            // Same day, check free count
            self.daily_draw_count < 1
        }
    }

    /// Update draw fortune count
    pub fn update_draw_count(&mut self) {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_draw_time >= 86400;

        if is_new_day {
            // New day, reset count
            self.daily_draw_count = 1;
        } else {
            // Same day, increase count
            self.daily_draw_count += 1;
        }

        self.last_draw_time = now;
        self.total_draws = self.total_draws.saturating_add(1);
    }

    /// Consume merit for extra draw fortune (using dynamic config cost)
    pub fn consume_merit_for_draw(&mut self, merit_cost: u64) -> Result<()> {
        if self.merit < merit_cost {
            return err!(ErrorCode::InsufficientMerit);
        }
        self.merit -= merit_cost;
        Ok(())
    }

    /// Update daily wish count
    pub fn update_wish_count(&mut self) {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_wish_time >= 86400;

        if is_new_day {
            // New day, reset count
            self.daily_wish_count = 1;
        } else {
            // Same day, increase count
            self.daily_wish_count += 1;
        }

        self.last_wish_time = now;
        self.total_wishes = self.total_wishes.saturating_add(1);
    }

    /// Wishing more than 3 times daily requires consuming merit
    pub fn consume_merit_for_wish(&mut self, merit_cost: u64) -> Result<()> {
        if self.merit < merit_cost {
            return err!(ErrorCode::InsufficientMerit);
        }
        self.merit -= merit_cost;
        Ok(())
    }

    /// Check if can wish for free
    pub fn can_wish_free(&self) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_wish_time >= 86400;

        if is_new_day {
            true // New day can wish for free
        } else {
            self.daily_wish_count < 3 // Same day check count
        }
    }
}

impl UserDonationState {
    pub const SEED_PREFIX: &str = "user_donation";

    /// Calculate level based on donation amount
    pub fn calculate_donation_level(&self) -> u8 {
        let donation_sol = self.donation_amount as f64 / 1_000_000_000.0; // Convert to SOL

        if donation_sol >= 5.0 {
            4 // Supreme Patron
        } else if donation_sol >= 1.0 {
            3 // Gold Protector
        } else if donation_sol >= 0.2 {
            2 // Silver Disciple
        } else if donation_sol >= 0.05 {
            1 // Bronze Believer
        } else {
            0 // No level
        }
    }

    /// Update donation level
    pub fn update_donation_level(&mut self) {
        self.donation_level = self.calculate_donation_level();
    }

    /// Check if can mint Buddha NFT for free (>0.5 SOL)
    pub fn can_mint_buddha_free(&self) -> bool {
        let donation_sol = self.donation_amount as f64 / 1_000_000_000.0;
        donation_sol >= 0.5
    }

    /// Get rewards by donation level (merit reward, temple incense points)
    pub fn get_donation_rewards(&self) -> (u64, u64) {
        // (merit, incense_points)
        match self.donation_level {
            1 => (65, 1200),       // Bronze Believer
            2 => (1300, 6300),     // Silver Disciple
            3 => (14000, 30000),   // Gold Protector
            4 => (120000, 100000), // Supreme Patron
            _ => (0, 0),           // No level
        }
    }

    /// Process donation logic
    pub fn process_donation(&mut self, amount_lamports: u64) {
        let now = Clock::get().unwrap().unix_timestamp;

        // Update donation amount
        self.donation_amount = self.donation_amount.saturating_add(amount_lamports);

        // Update donation statistics
        self.total_donation_count = self.total_donation_count.saturating_add(1);
        self.last_donation_time = now;

        // 更新等级
        self.update_donation_level();
    }
}
