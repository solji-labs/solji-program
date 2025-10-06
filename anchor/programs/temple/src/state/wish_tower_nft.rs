use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct WishTowerNFT {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub tower_id: u64,
    pub level: u8,
    pub wish_count: u8,
    pub minted_at: i64,
}

impl WishTowerNFT {
    pub const TOKEN_DECIMALS: u8 = 0;
    pub const TOKEN_SYMBOL: &'static str = "WISH_TOWER";
}
