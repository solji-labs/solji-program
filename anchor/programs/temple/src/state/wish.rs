use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Wish {
    pub id: u64,                // id
    pub creator: Pubkey,        // Creator
    pub content_hash: [u8; 32], // IPFS content hash
    pub is_anonymous: bool,     // Anonymous or public
    pub created_at: i64,        // Created time
    pub likes: u64,             // Like count
    pub bump: u8,
}

impl Wish {
    pub const SEED_PREFIX: &'static str = "wish";
}
