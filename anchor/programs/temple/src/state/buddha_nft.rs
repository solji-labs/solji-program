use anchor_lang::prelude::*;

/// Buddha NFT
#[account]
#[derive(Default, Debug, InitSpace)]
pub struct BuddhaNFT {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub serial_number: u32,
    pub minted_at: i64,
    pub is_active: bool,
}

impl BuddhaNFT {
    pub const SEED_PREFIX: &'static str = "BuddhaNFT";
    pub const TOKEN_DECIMALS: u8 = 0;
    pub const TOKEN_NAME: &'static str = "BuddhaNFT";
    pub const TOKEN_SYMBOL: &'static str = "MTK";
    pub const TOKEN_URL: &'static str =
        "https://solji.mypinata.cloud/ipfs/QmYHsbSuCpSUMksQP89VHawf21iJ7K6UE899uBrUmJgbVg";
}
