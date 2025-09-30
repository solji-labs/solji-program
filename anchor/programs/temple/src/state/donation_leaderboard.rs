use anchor_lang::prelude::*;

// Donation leaderboard user entry
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DonationUser {
    pub user: Pubkey,
    pub total_donation: u64, // Total donation amount (lamports)
}

// Donation leaderboard account
#[account]
#[derive(InitSpace)]
pub struct DonationLeaderboard {
    pub bump: u8,
    pub total_donors: u32,            // Total number of donors
    pub donation_deadline: u64,       // Donation deadline timestamp
    pub distribution_completed: bool, // Whether distribution is completed
    pub distributed_count: u32,       // Number of NFTs distributed
    pub last_updated: i64,            // Last updated time
    // Store top 10,000 donors (sorted by donation amount descending)
    #[max_len(10000)]
    pub top_donors: Vec<DonationUser>,
}

impl DonationLeaderboard {
    pub const SEED_PREFIX: &'static str = "donation_leaderboard";

    // Initialize leaderboard
    pub fn initialize(&mut self, bump: u8, donation_deadline: u64) {
        self.bump = bump;
        self.total_donors = 0;
        self.donation_deadline = donation_deadline;
        self.distribution_completed = false;
        self.distributed_count = 0;
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    // Update user donation record
    pub fn update_donation(&mut self, user: Pubkey, donation_amount: u64) {
        let now = Clock::get().unwrap().unix_timestamp;

        // Check if before deadline
        if (now as u64) >= self.donation_deadline {
            return; // Do not update leaderboard after deadline
        }

        // Find existing entry
        if let Some(existing_entry) = self.top_donors.iter_mut().find(|u| u.user == user) {
            // Update existing user's donation amount
            existing_entry.total_donation = existing_entry
                .total_donation
                .saturating_add(donation_amount);
        } else {
            // Add new user
            self.top_donors.push(DonationUser {
                user,
                total_donation: donation_amount,
            });
            self.total_donors = self.total_donors.saturating_add(1);
        }

        // Re-sort (descending)
        self.top_donors
            .sort_by(|a, b| b.total_donation.cmp(&a.total_donation));

        // Keep top 10,000
        if self.top_donors.len() > 10000 {
            self.top_donors.truncate(10000);
        }

        self.last_updated = now;
    }

    // Get user rank (return rank position, 0-based)
    pub fn get_user_rank(&self, user: &Pubkey) -> Option<u32> {
        self.top_donors
            .iter()
            .position(|u| u.user == *user)
            .map(|pos| pos as u32)
    }

    // Check if user is in top 10,000
    pub fn is_top_donor(&self, user: &Pubkey) -> bool {
        self.get_user_rank(user).is_some()
    }

    // Get top N donors
    pub fn get_top_donors(&self, limit: usize) -> &[DonationUser] {
        let len = self.top_donors.len().min(limit);
        &self.top_donors[..len]
    }

    // Mark distribution completed
    pub fn mark_distribution_completed(&mut self, distributed_count: u32) {
        self.distribution_completed = true;
        self.distributed_count = distributed_count;
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    // Check if can start distribution
    pub fn can_start_distribution(&self, current_time: u64) -> bool {
        current_time >= self.donation_deadline && !self.distribution_completed
    }

    // Get list of users to distribute to (top 10,000 without Buddha NFT)
    pub fn get_eligible_donors(&self, users_with_buddha: &[Pubkey]) -> Vec<DonationUser> {
        self.top_donors
            .iter()
            .filter(|donor| !users_with_buddha.contains(&donor.user))
            .take(10000)
            .cloned()
            .collect()
    }
}
