

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Wish {
    /// 愿望唯一ID
    pub wish_id: u64,
    
    /// 愿望创建者地址
    pub creator: Pubkey,
    
    /// IPFS内容哈希 (32字节)
    /// 存储愿望的实际内容，包括文本、图片等
    pub content_hash: [u8; 32],
    
    /// 是否匿名愿望
    /// true: 匿名模式，不显示创建者信息
    /// false: 公开模式，显示创建者地址
    pub is_anonymous: bool,
    
    /// 创建时间戳 (Unix时间戳)
    pub created_at: i64,
    
    /// 点赞数量
    /// 记录该愿望被点赞的总次数
    pub total_likes: u64,

    /// 是否掉落御守
    pub is_amulet_dropped: bool,
     
}


impl Wish {
    
    pub const SEED_PREFIX: &'static str = "wish_v1";

    pub const KARMA_COST_PER_WISH: u64 = 5;

    pub fn initialize(&mut self, wish_id: u64, creator: Pubkey, content_hash: [u8; 32], is_amulet_dropped: bool,is_anonymous: bool,created_at: i64) -> Result<()> {
        self.wish_id = wish_id;
        self.creator = creator;
        self.content_hash = content_hash;
        self.is_anonymous = is_anonymous;
        self.created_at = created_at; 
        self.total_likes = 0;
        self.is_amulet_dropped = is_amulet_dropped;
        Ok(())
    }

    pub fn add_like(&mut self) -> Result<()> {
        self.total_likes = self.total_likes.checked_add(1).unwrap_or(u64::MAX);
        Ok(())
    }

    pub fn cancel_like(&mut self) -> Result<()> {
 
        self.total_likes = self.total_likes.checked_sub(1).unwrap_or(u64::MAX);
        Ok(())
    }
}


#[account]
#[derive(InitSpace)]
pub struct WishLike {
    /// 愿望唯一ID
    pub wish_id: u64,
    
    /// 愿望创建者地址
    pub creator: Pubkey,

    /// 点赞者地址
    pub liker: Pubkey,
    
    /// 点赞时间戳 (Unix时间戳)
    pub created_at: i64,
}

impl WishLike {
    pub const SEED_PREFIX: &'static str = "wish_like_v1";


    pub fn initialize(&mut self, wish_id: u64, creator: Pubkey, liker: Pubkey, created_at: i64) -> Result<()> {
        self.wish_id = wish_id;
        self.creator = creator;
        self.liker = liker;
        self.created_at = created_at;
        Ok(())
    }
}





#[error_code]
pub enum WishError {
    #[msg("Wish like already exists")]
    WishLikeAlreadyExists,
    #[msg("Invalid creator")]
    InvalidCreator,
    #[msg("Invalid liker")]
    InvalidLiker,
    #[msg("Invalid wish")]
    InvalidWish,
}