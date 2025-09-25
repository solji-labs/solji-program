use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AmuletNFT {
    // 所有者地址
    pub owner: Pubkey,
    // NFT铸造地址
    pub mint: Pubkey,
    // 御守名称
    #[max_len(50)]
    pub name: String,
    // 御守描述
    #[max_len(200)]
    pub description: String,
    // 铸造时间
    pub minted_at: i64,
    // 来源（0=抽签获得，1=许愿获得）
    pub source: u8,
    // 序列号
    pub serial_number: u32,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum AmuletSource {
    DrawFortune, // 抽签获得
    MakeWish,    // 许愿获得
}

impl AmuletNFT {
    pub const SEED_PREFIX: &'static str = "amulet_nft";
    pub const TOKEN_DECIMALS: u8 = 0;

    // 获取御守URI
    pub fn get_amulet_uri(&self) -> String {
        format!(
            "https://api.foxverse.co/temple/amulet/{}/metadata.json",
            self.serial_number
        )
    }

    // 获取来源描述
    pub fn get_source_description(&self) -> &'static str {
        match self.source {
            0 => "抽签获得",
            1 => "许愿获得",
            _ => "未知来源",
        }
    }
}

// 用户御守收藏状态
#[account]
#[derive(InitSpace)]
pub struct UserAmuletCollection {
    pub user: Pubkey,
    // 拥有的御守数量统计
    pub total_amulets: u32,
    // 各来源御守数量
    pub draw_fortune_count: u32, // 抽签获得的数量
    pub make_wish_count: u32,    // 许愿获得的数量
    // 可铸造御守余额（概率掉落获得的）
    pub pending_amulets: u32, // 可铸造的御守数量
    // 最后更新时间
    pub last_updated: i64,
    pub bump: u8,
}

impl UserAmuletCollection {
    pub const SEED_PREFIX: &'static str = "user_amulets";

    // 更新御守统计
    pub fn update_stats(&mut self, source: u8, increment: bool) {
        let delta = if increment { 1i32 } else { -1i32 };

        // 更新总数
        self.total_amulets = (self.total_amulets as i32 + delta) as u32;

        // 更新来源统计
        match source {
            0 => {
                // DrawFortune
                self.draw_fortune_count = (self.draw_fortune_count as i32 + delta) as u32;
            }
            1 => {
                // MakeWish
                self.make_wish_count = (self.make_wish_count as i32 + delta) as u32;
            }
            _ => {} // 其他来源不统计
        }

        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }
}
