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

// User amulet collection status
#[account]
#[derive(InitSpace)]
pub struct UserAmuletCollection {
    pub user: Pubkey,
    // Total amulets owned statistics
    pub total_amulets: u32,
    // Amulet count by source
    pub draw_fortune_count: u32, // Count obtained from drawing fortune
    pub make_wish_count: u32,    // Count obtained from making wish
    // Pending amulets balance for minting (obtained from probability drops)
    pub pending_amulets: u32, // Number of amulets that can be minted
    // Last updated time
    pub last_updated: i64,
    pub bump: u8,
}

impl UserAmuletCollection {
    pub const SEED_PREFIX: &'static str = "user_amulets";

    // Update amulet statistics
    pub fn update_stats(&mut self, source: u8, increment: bool) {
        let delta = if increment { 1i32 } else { -1i32 };

        // Update total count
        self.total_amulets = (self.total_amulets as i32 + delta) as u32;

        // Update source statistics
        match source {
            0 => {
                // DrawFortune
                self.draw_fortune_count = (self.draw_fortune_count as i32 + delta) as u32;
            }
            1 => {
                // MakeWish
                self.make_wish_count = (self.make_wish_count as i32 + delta) as u32;
            }
            _ => {} // Other sources not counted
        }

        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }
}
