use anchor_lang::prelude::*;

use crate::global_error::GlobalError;

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
