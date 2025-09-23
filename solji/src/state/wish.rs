use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Wish {
    pub id: u64,                // id
    pub creator: Pubkey,        // 创建人
    pub content_hash: [u8; 32], // IPFS内容hash
    pub is_anonymous: bool,     // 匿名或者公开
    pub created_at: i64,        // 创建时间
    pub likes: u64,             // 点赞数
    pub bump: u8,
}

impl Wish {
    pub const SEED_PREFIX: &'static str = "wish";
}
