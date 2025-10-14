// Temple status bit index
#[derive(Clone, Copy, Debug)]
pub enum TempleStatusBitIndex {
    BuyIncense = 0,
    BurnIncense = 1,
    DrawFortune = 2,
    CreateWish = 3,
    Donate = 4,
    MintNFT = 5,
}

use crate::state::global_stats::GlobalStats;
use crate::state::shop_item::{ShopItem, ShopItemType};
use anchor_lang::prelude::*;

// ===== Core dynamic configuration =====

// 1. Incense type configuration
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct IncenseType {
    pub id: u8, // Incense type ID
    #[max_len(10)]
    pub name: String, // Name
    pub price_lamports: u64, // Price per incense stick
    pub merit: u64, // Merit value
    pub incense_points: u64, // Incense points
    pub is_donation: bool, // Whether it's donation incense
}

// 2. Fortune drawing configuration
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct FortuneConfig {
    pub great_luck_prob: u8,     // Great luck probability (0-100)
    pub good_luck_prob: u8,      // Good luck probability (0-100)
    pub neutral_prob: u8,        // Neutral probability (0-100)
    pub bad_luck_prob: u8,       // Bad luck probability (0-100)
    pub great_bad_luck_prob: u8, // Great bad luck probability (0-100)
}

// 3. Donation level configuration
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct DonationLevelConfig {
    pub level: u8,           // Level (1-4)
    pub min_amount_sol: f64, // Minimum amount (SOL)
    pub merit_reward: u64,   // Merit reward
    pub incense_reward: u64, // Incense reward
}

// 4. Donation reward configuration
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct DonationRewardConfig {
    pub min_donation_sol: f64,       // Minimum donation amount (SOL)
    pub incense_id: u8,              // Reward incense type ID
    pub incense_amount: u64,         // Reward incense amount
    pub burn_bonus_per_001_sol: u64, // Burn bonus per 0.01 SOL
}

// 5. Special incense types (obtained through donations)
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct SpecialIncenseType {
    pub id: u8, // Incense type ID
    #[max_len(20)]
    pub name: String, // Name
    pub required_donation_sol: f64, // Required donation amount (SOL)
    pub amount_per_donation: u64, // Amount received per donation milestone
    pub merit: u64, // Merit value
    pub incense_points: u64, // Incense points
    pub is_donation_only: bool, // Whether only obtainable through donation
}

// 5. Temple level configuration
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct TempleLevelConfig {
    pub level: u8,                    // Level
    pub required_incense_points: u64, // Required incense points
    pub required_draw_fortune: u64,   // Required draw fortune count
    pub required_wishes: u64,         // Required wishes
    pub required_donations_sol: f64,  // Required donation amount (SOL)
    pub required_fortune_nfts: u64,   // Required fortune NFTs
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct DynamicConfig {
    // 1. Incense type configuration
    #[max_len(10)]
    pub incense_types: Vec<IncenseType>,

    // 2. Fortune drawing configuration
    pub regular_fortune: FortuneConfig, // Regular user probability
    pub buddha_fortune: FortuneConfig,  // Buddha NFT holder probability

    // 3. Donation level configuration
    #[max_len(4)]
    pub donation_levels: Vec<DonationLevelConfig>,

    // 4. Donation reward configuration
    #[max_len(10)]
    pub donation_rewards: Vec<DonationRewardConfig>,

    // 5. Temple level configuration
    #[max_len(4)]
    pub temple_levels: Vec<TempleLevelConfig>,

    // 6. Special incense types
    #[max_len(2)]
    pub special_incense_types: Vec<SpecialIncenseType>,
}

// Temple config - main account, responsible for configuration and core status
#[account]
#[derive(InitSpace)]
pub struct TempleConfig {
    // Admin configuration
    pub owner: Pubkey,    // Temple admin address
    pub treasury: Pubkey, // Temple treasury address

    // Core status (requires signature permission)
    pub level: u8,             // Current temple level, calculated in real-time
    pub created_at: i64,       // Creation time
    pub total_buddha_nft: u32, // Buddha NFT count (minting permission)
    pub total_medal_nft: u32,  // Medal NFT count (minting permission)
    pub total_amulets: u32,    // Amulet count (minting permission)

    // Control configuration
    pub status: u8, // Status bit control, 0 enables all, other values disable corresponding functions by bit
    pub open_time: u64, // Launch timestamp
    pub donation_deadline: u64, // Donation deadline timestamp, for Buddha NFT distribution

    // All configurations are placed in dynamic config
    pub dynamic_config: DynamicConfig,
}

impl TempleConfig {
    pub const SEED_PREFIX: &str = "temple_v1";

    // Get incense type
    pub fn find_incense_type(&self, id: u8) -> Option<&IncenseType> {
        self.dynamic_config
            .incense_types
            .iter()
            .find(|t| t.id == id)
    }

    // Get special incense type
    pub fn find_special_incense_type(&self, id: u8) -> Option<&SpecialIncenseType> {
        self.dynamic_config
            .special_incense_types
            .iter()
            .find(|t| t.id == id)
    }

    // Get incense price
    pub fn get_fee_per_incense(&self, incense_id: u8) -> u64 {
        self.find_incense_type(incense_id)
            .map(|t: &IncenseType| t.price_lamports)
            .unwrap_or(0)
    }

    // Get fortune probability configuration
    pub fn get_fortune_config(&self, has_buddha_nft: bool) -> &FortuneConfig {
        if has_buddha_nft {
            &self.dynamic_config.buddha_fortune
        } else {
            &self.dynamic_config.regular_fortune
        }
    }

    // Get donation level configuration
    pub fn get_donation_level_config(&self, level: u8) -> Option<&DonationLevelConfig> {
        self.dynamic_config
            .donation_levels
            .iter()
            .find(|d| d.level == level)
    }

    // Check if incense type exists
    pub fn is_incense_available(&self, incense_id: u8) -> bool {
        self.find_incense_type(incense_id).is_some()
    }

    // Dynamically calculate level
    pub fn calculate_temple_level(&self, global_stats: &GlobalStats) -> u8 {
        let incense_points = global_stats.total_incense_points;
        let donations_sol = global_stats.total_donations_sol();

        // Match level requirements
        for level_config in self.dynamic_config.temple_levels.iter().rev() {
            if incense_points >= level_config.required_incense_points
                && global_stats.total_draw_fortune >= level_config.required_draw_fortune
                && global_stats.total_wishes >= level_config.required_wishes
                && donations_sol >= level_config.required_donations_sol
                && global_stats.total_fortune_nfts >= level_config.required_fortune_nfts
            {
                return level_config.level;
            }
        }

        1
    }

    // Update temple level
    pub fn update_level(&mut self, global_stats: &GlobalStats) {
        self.level = self.calculate_temple_level(global_stats);
    }

    // Status management methods
    pub fn get_status_by_bit(&self, bit: TempleStatusBitIndex) -> bool {
        let status = 1u8 << (bit as u8);
        (self.status & status) == 0
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = status;
    }

    pub fn set_status_by_bit(&mut self, bit: TempleStatusBitIndex, disabled: bool) {
        let mask = 1u8 << (bit as u8);
        if disabled {
            self.status |= mask; // Set bit to 1 (disable)
        } else {
            self.status &= !mask; // Clear bit to 0 (enable)
        }
    }

    // Whether can perform operation, need to verify both time and function
    pub fn can_perform_operation(
        &self,
        bit: TempleStatusBitIndex,
        current_time: u64,
    ) -> Result<()> {
        // Launch time
        if current_time < self.open_time {
            return err!(crate::error::ErrorCode::NotApproved);
        }

        // Function status
        if !self.get_status_by_bit(bit) {
            return err!(crate::error::ErrorCode::NotApproved);
        }

        Ok(())
    }
}
