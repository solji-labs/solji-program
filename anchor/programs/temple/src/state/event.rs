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

// WishCreated Events
#[event]
pub struct WishCreated {
    pub user: Pubkey,
    pub wish_id: u64,
    pub is_anonymous: bool,
    pub amulet_dropped: bool,
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
