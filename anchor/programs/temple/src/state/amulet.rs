use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AmuletNFT {
    // Owner address
    pub owner: Pubkey,
    // NFT mint address
    pub mint: Pubkey,
    // Amulet name
    #[max_len(50)]
    pub name: String,
    // Amulet description
    #[max_len(200)]
    pub description: String,
    // Minted time
    pub minted_at: i64,
    // Source (0=obtained from drawing fortune, 1=obtained from making wish)
    pub source: u8,
    // Serial number
    pub serial_number: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum AmuletSource {
    DrawFortune, // Obtained from drawing fortune
    MakeWish,    // Obtained from making wish
}

impl AmuletNFT {
    pub const SEED_PREFIX: &'static str = "amulet_nft";
    pub const TOKEN_DECIMALS: u8 = 0;

    // Get amulet URI
    pub fn get_amulet_uri(&self) -> String {
        format!(
            "https://api.foxverse.co/temple/amulet/{}/metadata.json",
            self.serial_number
        )
    }

    // Get source description
    pub fn get_source_description(&self) -> &'static str {
        match self.source {
            0 => "Obtained from drawing fortune",
            1 => "Obtained from making wish",
            _ => "Unknown source",
        }
    }
}

