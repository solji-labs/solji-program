use anchor_lang::prelude::*;

use crate::{global_error::GlobalError, states::CreateNftArgs};

#[account]
#[derive(InitSpace)]
pub struct WishUser {
    pub user: Pubkey,
    // 许愿次数
    pub total_count: u8,
    // 许愿时间
    pub update_time: i64,
    // 是否免费
    pub daily_count: u8,
}

impl WishUser {
    pub const WISH_FEE: u8 = 5;

    pub const NAME: &str = "Ema NFT";
    pub const SYMBOL: &str = "Ema";
    pub const URL: &str = "https://solji.io/";

    pub fn new(user: Pubkey) -> Self {
        Self {
            total_count: 0,
            update_time: Clock::get().unwrap().unix_timestamp,
            daily_count: 0,
            user,
        }
    }

    pub fn update_user_wish_count(&mut self) -> Result<()> {
        self.total_count = self
            .total_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.daily_count = self
            .daily_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.update_time = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct PublishWish {
    #[max_len(100)]
    pub content: String,
    // 作者
    pub author: Pubkey,
    // 创建时间
    pub create_time: i64,
    // 点赞
    pub like_count: u64,
}

impl PublishWish {
    pub fn new(author: Pubkey, content: String) -> Self {
        Self {
            content,
            author,
            create_time: Clock::get().unwrap().unix_timestamp,
            like_count: 0,
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct WishLike {
    // 点赞的人
    pub like_pubkey: Pubkey,
    // 点赞的愿望
    pub with_pubkey: Pubkey,
}

impl WishLike {
    pub fn new(like_pubkey: Pubkey, with_pubkey: Pubkey) -> Self {
        Self {
            like_pubkey,
            with_pubkey,
        }
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tower {
    SeedTower,
    BasicTower,
    AdvancedTower,
    GrandWishTower,
    PerfectionTower,
}

impl Tower {
    pub fn get_tower(wish_count: &u32) -> Self {
        match wish_count {
            0..=9 => Tower::SeedTower,
            10..=49 => Tower::BasicTower,
            50..=199 => Tower::AdvancedTower,
            200..=499 => Tower::GrandWishTower,
            _ => Tower::PerfectionTower,
        }
    }

    pub fn get_level(&self) -> i8 {
        match self {
            Tower::SeedTower => 0,
            Tower::BasicTower => 1,
            Tower::AdvancedTower => 2,
            Tower::GrandWishTower => 3,
            _ => 4,
        }
    }
    pub fn get_nft_args(&self) -> CreateNftArgs {
        match self {
            Tower::SeedTower => CreateNftArgs {
                name: "Seed Tower".to_string(),
                symbol: "Seed".to_string(),
                url: "https://solji.io/".to_string(),
                is_mutable: true,
                collection_details: true,
            },
            Tower::BasicTower => CreateNftArgs {
                name: "Basic Tower".to_string(),
                symbol: "Basic".to_string(),
                url: "https://solji.io/".to_string(),
                is_mutable: true,
                collection_details: true,
            },
            Tower::AdvancedTower => CreateNftArgs {
                name: "Advanced Tower".to_string(),
                symbol: "Advanced".to_string(),
                url: "https://solji.io/".to_string(),
                is_mutable: true,
                collection_details: true,
            },
            Tower::GrandWishTower => CreateNftArgs {
                name: "Grand Wish Tower".to_string(),
                symbol: "Grand".to_string(),
                url: "https://solji.io/".to_string(),
                is_mutable: true,
                collection_details: true,
            },
            Tower::PerfectionTower => CreateNftArgs {
                name: "Perfection Tower".to_string(),
                symbol: "Perfection".to_string(),
                url: "https://solji.io/".to_string(),
                is_mutable: true,
                collection_details: true,
            },
        }
    }
}
