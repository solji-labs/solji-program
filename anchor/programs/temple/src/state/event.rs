use anchor_lang::prelude::*;

// Donation Events

#[event]
pub struct DonationCompleted {
    pub user: Pubkey,
    pub amount: u64,
    pub total_donated: u64,
    pub level: u8,
    pub timestamp: i64,
}

#[event]
pub struct RewardsProcessed {
    pub user: Pubkey,
    pub merit_reward: u64,
    pub incense_points_reward: u64,
    pub timestamp: i64,
}

#[event]
pub struct DonationNFTMinted {
    pub user: Pubkey,
    pub nft_mint: Pubkey,
    pub level: u8,
    pub serial_number: u32,
    pub timestamp: i64,
}

// FortuneDrawn Events

#[event]
pub struct FortuneDrawn {
    pub user: Pubkey,
    pub fortune_result: String,
    pub used_merit: bool,
    pub amulet_dropped: bool,
    pub timestamp: i64,
}

// Amulet Dropped Events
#[event]
pub struct AmuletDropped {
    pub user: Pubkey,
    pub source: String, // "fortune" or "wish"
    pub timestamp: i64,
}

// AmuletMinted Events
#[event]
pub struct AmuletMinted {
    pub user: Pubkey,
    pub amulet_mint: Pubkey,
    pub source: String,
    pub serial_number: u32,
    pub timestamp: i64,
}

// WishCreated Events
#[event]
pub struct WishCreated {
    pub user: Pubkey,
    pub wish_id: u64,
    pub content_hash: [u8; 32],
    pub is_anonymous: bool,
    pub amulet_dropped: bool,
    pub timestamp: i64,
}

// WishTower Events
#[event]
pub struct WishTowerCreated {
    pub user: Pubkey,
    pub tower_id: u64,
    pub max_level: u8,
    pub timestamp: i64,
}

#[event]
pub struct WishAddedToTower {
    pub user: Pubkey,
    pub tower_id: u64,
    pub wish_id: u64,
    pub level: u8,
    pub level_completed: bool,
    pub tower_completed: bool,
    pub timestamp: i64,
}

#[event]
pub struct WishTowerUpdated {
    pub user: Pubkey,
    pub wish_count: u32,
    pub level: u8,
    pub timestamp: i64,
}

#[event]
pub struct WishTowerNFTMinted {
    pub user: Pubkey,
    pub nft_mint: Pubkey,
    pub wish_count: u32,
    pub level: u8,
    pub timestamp: i64,
}

// BurnIncense Events
#[event]
pub struct IncenseBurned {
    pub user: Pubkey,
    pub incense_id: u8,
    pub amount: u64,
    pub timestamp: i64,
}

// ShopConfig Events
#[event]
pub struct ShopConfigUpdated {
    pub shop_config: Pubkey,
    pub temple_config: Pubkey,
    pub owner: Pubkey,
    pub shop_items: Vec<crate::state::shop_item::ShopItem>,
    pub timestamp: i64,
}
