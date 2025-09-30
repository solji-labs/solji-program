use anchor_lang::prelude::*;

// Leaderboard user entry
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LeaderboardUser {
    pub user: Pubkey,
    pub value: u64,
}

// Leaderboard entry account (store detailed scores)
#[account]
#[derive(InitSpace)]
pub struct LeaderboardEntry {
    pub bump: u8,
    pub user: Pubkey,              // User address
    pub period: LeaderboardPeriod, // Leaderboard period
    pub incense_count: u64,        // Incense burn count
    pub incense_value: u64,        // Incense points
    pub merit: u64,                // Merit
    pub last_updated: i64,         // Last updated time
}

impl LeaderboardEntry {
    pub const SEED_PREFIX: &'static str = "leaderboard_entry";

    pub fn new(user: Pubkey, period: LeaderboardPeriod, bump: u8) -> Self {
        let now = Clock::get().unwrap().unix_timestamp;
        Self {
            bump,
            user,
            period,
            incense_count: 0,
            incense_value: 0,
            merit: 0,
            last_updated: now,
        }
    }

    pub fn update(&mut self, incense_count: u64, incense_value: u64, merit: u64) {
        self.incense_count = self.incense_count.saturating_add(incense_count);
        self.incense_value = self.incense_value.saturating_add(incense_value);
        self.merit = self.merit.saturating_add(merit);
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    pub fn reset(&mut self) {
        self.incense_count = 0;
        self.incense_value = 0;
        self.merit = 0;
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }
}

// Main leaderboard account (store sorted user addresses and values)
#[account]
#[derive(InitSpace)]
pub struct Leaderboard {
    pub bump: u8,
    pub total_users: u32,        // Total users
    pub last_daily_reset: i64,   // Daily reset time
    pub last_weekly_reset: i64,  // Weekly reset time
    pub last_monthly_reset: i64, // Monthly reset time
    // Store top 10 user addresses and values (sorted)
    #[max_len(10)]
    pub daily_users: Vec<LeaderboardUser>, // Daily active users list
    #[max_len(10)]
    pub weekly_users: Vec<LeaderboardUser>, // Weekly active users list
    #[max_len(10)]
    pub monthly_users: Vec<LeaderboardUser>, // Monthly active users list
}

impl Leaderboard {
    pub const SEED_PREFIX: &'static str = "leaderboard";

    // Initialize leaderboard
    pub fn initialize(&mut self, bump: u8) {
        self.bump = bump;
        self.total_users = 0;
        self.last_daily_reset = Clock::get().unwrap().unix_timestamp;
        self.last_weekly_reset = self.last_daily_reset;
        self.last_monthly_reset = self.last_daily_reset;
    }

    // Check if need to reset leaderboard periods
    pub fn check_and_reset_periods(&mut self) {
        let now = Clock::get().unwrap().unix_timestamp;

        // Daily reset (24 hours)
        if now - self.last_daily_reset >= 24 * 60 * 60 {
            self.last_daily_reset = now;
            self.daily_users.clear();
        }

        // Weekly reset (7 days)
        if now - self.last_weekly_reset >= 7 * 24 * 60 * 60 {
            self.last_weekly_reset = now;
            self.weekly_users.clear();
        }

        // Monthly reset (30 days)
        if now - self.last_monthly_reset >= 30 * 24 * 60 * 60 {
            self.last_monthly_reset = now;
            self.monthly_users.clear();
        }
    }

    // Update user's position in leaderboard
    pub fn update_user_ranking(&mut self, user: Pubkey, value: u64, period: LeaderboardPeriod) {
        let user_list = match period {
            LeaderboardPeriod::Daily => &mut self.daily_users,
            LeaderboardPeriod::Weekly => &mut self.weekly_users,
            LeaderboardPeriod::Monthly => &mut self.monthly_users,
        };

        // Remove existing entry
        user_list.retain(|u| u.user != user);

        // Insert new entry
        user_list.push(LeaderboardUser { user, value });

        // Sort (descending)
        user_list.sort_by(|a, b| b.value.cmp(&a.value));

        // Keep top 10
        if user_list.len() > 10 {
            user_list.truncate(10);
        }
    }

    // Get user rank (return rank position, 0-based)
    pub fn get_incense_leaderboard(&self, user: &Pubkey, period: LeaderboardPeriod) -> Option<u32> {
        let user_list = match period {
            LeaderboardPeriod::Daily => &self.daily_users,
            LeaderboardPeriod::Weekly => &self.weekly_users,
            LeaderboardPeriod::Monthly => &self.monthly_users,
        };

        user_list
            .iter()
            .position(|u| u.user == *user)
            .map(|pos| pos as u32)
    }

    // Check if user has visual effect reward (top 3)
    pub fn has_visual_effect(&self, user: &Pubkey, period: LeaderboardPeriod) -> bool {
        if let Some(rank) = self.get_incense_leaderboard(user, period) {
            rank <= 3
        } else {
            false
        }
    }
}

// Leaderboard period enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
pub enum LeaderboardPeriod {
    Daily,
    Weekly,
    Monthly,
}
