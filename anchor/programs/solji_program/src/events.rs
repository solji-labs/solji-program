use anchor_lang::prelude::*;

use crate::states::{IncenseType, LotteryType};

#[event]
pub struct DonateEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub merit_value: u64,
    pub incense_value: u64,
    pub timestamp: i64,
}

#[event]
pub struct MedalMintedEvent {
    pub user: Pubkey,
    pub level: String,
    pub nft_mint: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct MedalUpgradedEvent {
    pub user: Pubkey,
    pub old_level: String,
    pub new_level: String,
    pub nft_mint: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct DrawLotsEvent {
    pub user: Pubkey,
    pub lottery_type: LotteryType,
    pub merit_change: u64,
    pub timestamp: i64,
}

#[event]
pub struct CoinFlipEvent {
    pub player: Pubkey,
    pub randomness_account: Pubkey,
    pub commit_slot: u64,
    pub timestamp: i64,
}

#[event]
pub struct DestroyEvent {
    pub user: Pubkey,   // 销毁发起者
    pub mint: Pubkey,   // 被销毁的 NFT mint
    pub timestamp: i64, // 销毁时间（秒）
}
#[event]
pub struct IncenseBoughtEvent {
    pub buyer: Pubkey,
    pub incense_type: IncenseType,
    pub number: u64,
    pub unit_price: u64,
    pub total_amount: u64,
    pub timestamp: i64,
}

#[event]
pub struct IncenseBurned {
    pub user: Pubkey,
    pub incense_type: IncenseType,
    pub nft_mint: Pubkey,
    pub incense_value: u64,
    pub merit_value: u64,
    pub timestamp: i64,
}

#[event]
pub struct TempleWithdrawal {
    pub admin: Pubkey,
    pub amount: u64,
    pub remaining_balance: u64, // 提现后 Temple 账户剩余 lamports
    pub timestamp: i64,
}

#[event]
pub struct LikeCreated {
    pub liker: Pubkey,       // 点赞人
    pub wish: Pubkey,        // 被点赞的愿望
    pub new_like_count: u64, // 点赞后的总数
    pub timestamp: i64,
}

#[event]
pub struct WishCreated {
    pub user: Pubkey,       // 谁许的愿
    pub content: String,    // 愿望内容
    pub value: u64,         // 花费的功德值
    pub is_anonymous: bool, // 是否匿名
}
