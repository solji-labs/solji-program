use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct MedalNFT {
    // 所有者地址
    pub owner: Pubkey,
    // NFT铸造地址
    pub mint: Pubkey,
    // 当前等级 (1铜牌, 2银牌, 3金牌, 4至尊)
    pub level: u8,
    // 总捐款金额 (lamports)
    pub total_donation: u64,
    // 发行时间
    pub minted_at: i64,
    // 最后升级时间
    pub last_upgrade: i64,
    // 勋章持有者功德值
    pub merit: u64,
    // 序列号
    pub serial_number: u32,
    // 质押开始时间
    pub staked_at: Option<i64>,
}

impl MedalNFT {
    pub const SEED_PREFIX: &'static str = "medal_nft";
    pub const TOKEN_DECIMALS: u8 = 0;

    // 根据等级获取勋章名称
    pub fn get_medal_name(&self) -> &'static str {
        match self.level {
            1 => "入门功德铜章",
            2 => "精进银章",
            3 => "护法金章",
            4 => "至尊龙章",
            _ => "寺庙勋章",
        }
    }

    // 根据等级获取勋章URI
    pub fn get_medal_uri(&self) -> String {
        format!(
            "https://api.foxverse.co/temple/medal/{}/metadata.json",
            self.level
        )
    }

    // 注册等级对应的最低捐款金额 (SOL)
    pub fn get_level_min_donation_sol(level: u8) -> f64 {
        match level {
            1 => 0.05, // 铜牌
            2 => 0.2,  // 银牌
            3 => 1.0,  // 金牌
            4 => 5.0,  // 至尊
            _ => f64::INFINITY,
        }
    }

    // 检查是否可以升级到指定等级
    pub fn can_upgrade_to(&self, new_level: u8, current_donation_sol: f64) -> bool {
        if new_level <= self.level {
            return false;
        }
        if new_level > 4 {
            return false;
        }
        // 检查捐款金额是否达到新等级要求
        let required_sol = Self::get_level_min_donation_sol(new_level);
        current_donation_sol >= required_sol
    }

    // 获取下一个升级等级
    pub fn get_next_upgrade_level(&self, current_donation_sol: f64) -> Option<u8> {
        for level in (self.level + 1)..=4 {
            if self.can_upgrade_to(level, current_donation_sol) {
                return Some(level);
            }
        }
        None
    }

    // 获取勋章的描述
    pub fn get_description(&self) -> String {
        format!(
            "寺庙{} - 持有功德:{}, 总捐款:{} SOL",
            self.get_medal_name(),
            self.merit,
            self.total_donation as f64 / 1_000_000_000.0
        )
    }
}
