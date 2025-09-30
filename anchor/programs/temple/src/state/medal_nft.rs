use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MedalNFT {
    // Owner address
    pub owner: Pubkey,
    // NFT mint address
    pub mint: Pubkey,
    // Current level (1 bronze, 2 silver, 3 gold, 4 supreme)
    pub level: u8,
    // Total donation amount (lamports)
    pub total_donation: u64,
    // Minted time
    pub minted_at: i64,
    // Last upgrade time
    pub last_upgrade: i64,
    // Medal holder merit
    pub merit: u64,
    // Serial number
    pub serial_number: u32,
    // Staking start time
    pub staked_at: Option<i64>,
}

impl MedalNFT {
    pub const SEED_PREFIX: &'static str = "medal_nft";
    pub const TOKEN_DECIMALS: u8 = 0;

    // Get medal name by level
    pub fn get_medal_name(&self) -> &'static str {
        match self.level {
            1 => "Entry Merit Bronze Medal",
            2 => "Diligent Silver Medal",
            3 => "Protector Gold Medal",
            4 => "Supreme Dragon Medal",
            _ => "Temple Medal",
        }
    }

    // Get medal URI by level
    pub fn get_medal_uri(&self) -> String {
        format!(
            "https://api.foxverse.co/temple/medal/{}/metadata.json",
            self.level
        )
    }

    // Minimum donation amount for each level (SOL)
    pub fn get_level_min_donation_sol(level: u8) -> f64 {
        match level {
            1 => 0.05, // Bronze
            2 => 0.2,  // Silver
            3 => 1.0,  // Gold
            4 => 5.0,  // Supreme
            _ => f64::INFINITY,
        }
    }

    // Check if can upgrade to specified level
    pub fn can_upgrade_to(&self, new_level: u8, current_donation_sol: f64) -> bool {
        if new_level <= self.level {
            return false;
        }
        if new_level > 4 {
            return false;
        }
        // Check if donation amount meets new level requirement
        let required_sol = Self::get_level_min_donation_sol(new_level);
        current_donation_sol >= required_sol
    }

    // Get next upgrade level
    pub fn get_next_upgrade_level(&self, current_donation_sol: f64) -> Option<u8> {
        for level in (self.level + 1)..=4 {
            if self.can_upgrade_to(level, current_donation_sol) {
                return Some(level);
            }
        }
        None
    }

    // Get medal description
    pub fn get_description(&self) -> String {
        format!(
            "Temple {} - Merit: {}, Total donation: {} SOL",
            self.get_medal_name(),
            self.merit,
            self.total_donation as f64 / 1_000_000_000.0
        )
    }
}
