use anchor_lang::prelude::*;

// ===== 捐助相关事件 =====

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

// ===== 其他业务事件 =====

// 这里可以继续添加其他模块的事件定义
// 例如：烧香事件、抽签事件、许愿事件等
