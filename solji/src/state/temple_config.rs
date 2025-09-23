use anchor_lang::prelude::*;

// ===== 核心动态配置：只包含三个主要部分 =====

// 1. 烧香香型配置
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct IncenseType {
    pub id: u8, // 香型ID
    #[max_len(32)]
    pub name: String, // 名称
    pub price_lamports: u64, // 单支香的价格
    pub merit: u64, // 功德值
    pub incense_points: u64, // 香火值
    pub is_donation: bool, // 是否捐助的香
}

// 2. 抽签签文配置
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct FortuneConfig {
    pub great_luck_prob: u8,     // 大吉概率 (0-100)
    pub good_luck_prob: u8,      // 中吉概率 (0-100)
    pub neutral_prob: u8,        // 平概率 (0-100)
    pub bad_luck_prob: u8,       // 凶概率 (0-100)
    pub great_bad_luck_prob: u8, // 大凶概率 (0-100)
}

// 3. 捐助等级配置
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct DonationLevelConfig {
    pub level: u8,           // 等级 (1-4)
    pub min_amount_sol: f64, // 最低金额 (SOL)
    pub merit_reward: u64,   // 功德奖励
    pub incense_reward: u64, // 香火奖励
}

// 精简的动态配置参数（只包含三个核心部分）
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct DynamicConfig {
    // 1. 烧香香型配置
    #[max_len(10)]
    pub incense_types: Vec<IncenseType>,

    // 2. 抽签签文配置
    pub regular_fortune: FortuneConfig, // 普通用户概率
    pub buddha_fortune: FortuneConfig,  // 佛像持有者概率

    // 3. 捐助等级配置
    #[max_len(4)]
    pub donation_levels: Vec<DonationLevelConfig>,
}

// 寺庙配置
#[account]
#[derive(InitSpace)]
pub struct TempleConfig {
    pub owner: Pubkey,             // 寺庙管理员地址
    pub treasury: Pubkey,          // 寺庙国库地址
    pub total_incense_points: u64, // 总香火值
    pub total_merit: u64,          // 总功德
    pub level: u8,                 // 寺庙等级
    pub created_at: i64,           // 创建时间
    pub total_buddha_nft: u32,     // 佛像数量

    // 所有配置都放在动态配置中
    pub dynamic_config: DynamicConfig,
}

impl TempleConfig {
    pub const SEED_PREFIX: &str = "temple_v1";

    // 获取香型类型（从动态配置中查找）
    pub fn find_incense_type(&self, id: u8) -> Option<&IncenseType> {
        self.dynamic_config
            .incense_types
            .iter()
            .find(|t| t.id == id)
    }

    // 获取香型价格
    pub fn get_fee_per_incense(&self, incense_id: u8) -> u64 {
        self.find_incense_type(incense_id)
            .map(|t: &IncenseType| t.price_lamports)
            .unwrap_or(0)
    }

    // 增加香火值和功德值
    pub fn add_incense_value_and_merit(&mut self, incense_value: u64, merit: u64) {
        self.total_incense_points = self
            .total_incense_points
            .checked_add(incense_value)
            .unwrap_or(self.total_incense_points);
        self.total_merit = self
            .total_merit
            .checked_add(merit)
            .unwrap_or(self.total_merit);
    }

    // 获取抽签概率配置
    pub fn get_fortune_config(&self, has_buddha_nft: bool) -> &FortuneConfig {
        if has_buddha_nft {
            &self.dynamic_config.buddha_fortune
        } else {
            &self.dynamic_config.regular_fortune
        }
    }

    // 获取捐助等级配置
    pub fn get_donation_level_config(&self, level: u8) -> Option<&DonationLevelConfig> {
        self.dynamic_config
            .donation_levels
            .iter()
            .find(|d| d.level == level)
    }

    // 检查香型是否存在
    pub fn is_incense_available(&self, incense_id: u8) -> bool {
        self.find_incense_type(incense_id).is_some()
    }
}
