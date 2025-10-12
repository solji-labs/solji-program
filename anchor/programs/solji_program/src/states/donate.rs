use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL};

use crate::{
    global_error::GlobalError,
    states::{IncenseType, UserInfo},
};

#[account]
#[derive(InitSpace)]
pub struct DonateRecord {
    //捐助人
    pub user: Pubkey,

    // 捐赠SOL
    pub amount: u64,

    // 捐助获得的功德值
    pub merit_value: u64,

    // 香火值
    pub incense_value: u64,

    // 创建时间
    pub create_at: i64,
}

#[account]
#[derive(InitSpace)]
pub struct DonateCounter {
    pub user: Pubkey, // 关联用户
    pub count: u32,   // 当前计数器，用于给每笔捐款分配index
}

impl DonateCounter {
    pub fn new(user: Pubkey) -> Self {
        Self { user, count: 0 }
    }

    pub fn increment(&mut self) -> Result<()> {
        self.count = self.count.checked_add(1).ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }
}

impl DonateRecord {
    pub fn new(user: Pubkey, amount: u64) -> Self {
        Self {
            user,
            amount,
            merit_value: 0,
            incense_value: 0,
            create_at: Clock::get().unwrap().unix_timestamp,
        }
    }

    pub fn update_rewards(&mut self, merit_value: u64, incense_value: u64) {
        self.merit_value = merit_value;
        self.incense_value = incense_value;
    }

    // 奖励函数：根据捐赠金额（单位 lamports）返回 (功能奖励, 香火点数)
    pub fn get_donation_rewards(amount: u64) -> (u64, u64) {
        let (merit_value, incense_value) = match amount {
            50_000_000..=199_999_999 => (65, 1200),    // ≥ 0.05 SOL
            200_000_000..=999_999_999 => (1300, 6300), // ≥ 0.2 SOL
            1_000_000_000..=4_999_999_999 => (14_000, 30_000), // ≥ 1 SOL
            5_000_000_000..=u64::MAX => (120_000, 100_000), // ≥ 5 SOL
            _ => (0, 0),
        };
        (merit_value, incense_value)
    }

    pub fn get_badge_level(total_donation: u64) -> MedalLevel {
        match total_donation {
            50_000_000..=199_999_999 => MedalLevel::Bronze,
            200_000_000..=999_999_999 => MedalLevel::Silver,
            1_000_000_000..=4_999_999_999 => MedalLevel::Gold,
            5_000_000_000..=u64::MAX => MedalLevel::Supreme,
            _ => MedalLevel::None,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum MedalLevel {
    None,
    Bronze,
    Silver,
    Gold,
    Supreme,
}

impl MedalLevel {
    pub fn get_nft_name(&self) -> String {
        match self {
            MedalLevel::None => "".to_string(),
            MedalLevel::Bronze => "Beginner Merit Bronze Medal".to_string(),
            MedalLevel::Silver => "Diligence Silver Medal".to_string(),
            MedalLevel::Gold => "Dharma Protector Gold Medal".to_string(),
            MedalLevel::Supreme => "Supreme Dragon Medal".to_string(),
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            MedalLevel::None => "".to_string(),
            MedalLevel::Bronze => "Bronze".to_string(),
            MedalLevel::Silver => "Silver".to_string(),
            MedalLevel::Gold => "Gold".to_string(),
            MedalLevel::Supreme => "Supreme".to_string(),
        }
    }

    pub fn get_nft_uri(&self) -> String {
        match self {
            MedalLevel::None => "".to_string(),
            MedalLevel::Bronze => "https://xxx/bronze.json".to_string(),
            MedalLevel::Silver => "https://xxx/silver.json".to_string(),
            MedalLevel::Gold => "https://xxx/gold.json".to_string(),
            MedalLevel::Supreme => "https://xxx/supreme.json".to_string(),
        }
    }

    pub fn get_badge_level_reward(&self) -> (u64, u64) {
        match self {
            MedalLevel::None => (0, 0),
            MedalLevel::Bronze => (65, 1200),
            MedalLevel::Silver => (1300, 6300),
            MedalLevel::Gold => (14000, 30000),
            MedalLevel::Supreme => (120000, 100000),
        }
    }
}

impl Space for MedalLevel {
    const INIT_SPACE: usize = 1; // enum 占 1 字节（u8）
}

pub fn calc_incense_rewards(donation_sol: u64, user_info: &mut UserInfo) -> Result<()> {
    if donation_sol < 5 * LAMPORTS_PER_SOL {
        let number = donation_sol / 10_000_000;
        user_info.update_incense_property_count(IncenseType::ClearIncense, number)?;
    } else if donation_sol <= 50 * LAMPORTS_PER_SOL {
        let units_of_5 = donation_sol / (5 * LAMPORTS_PER_SOL); // 每 5 SOL 一个单位（向下取整）
        let secret = units_of_5 * 10;
        msg!("secret: {}", secret);
        user_info.update_incense_property_count(IncenseType::SecretBrewIncense, secret)?;
    } else {
        let units_of_50 = donation_sol / (50 * LAMPORTS_PER_SOL); // 每 50 SOL 一个单位（向下取整）
        let celestial = units_of_50 * 5;
        msg!("celestial: {}", celestial);
        user_info.update_incense_property_count(IncenseType::CelestialIncense, celestial)?;
    }
    Ok(())
}
