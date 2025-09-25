// 寺庙状态位索引
#[derive(Clone, Copy, Debug)]
pub enum TempleStatusBitIndex {
    BuyIncense = 0,
    BurnIncense = 1,
    DrawFortune = 2,
    CreateWish = 3,
    Donate = 4,
    MintNFT = 5,
}

use crate::state::global_stats::GlobalStats;
use crate::state::shop_item::{ShopItem, ShopItemType};
use anchor_lang::prelude::*;

// ===== 核心动态配置 =====

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

// 4. 捐助奖励配置
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct DonationRewardConfig {
    pub min_donation_sol: f64,       // 最低捐助金额 (SOL)
    pub incense_id: u8,              // 奖励香类型ID
    pub incense_amount: u64,         // 奖励香数量
    pub burn_bonus_per_001_sol: u64, // 每0.01SOL增加的烧香次数
}

// 5. 寺庙等级配置
#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct TempleLevelConfig {
    pub level: u8,                    // 等级
    pub required_incense_points: u64, // 需要香火值
    pub required_draw_fortune: u64,   // 需要抽签次数
    pub required_wishes: u64,         // 需要许愿次数
    pub required_donations_sol: f64,  // 需要捐助金额 (SOL)
    pub required_fortune_nfts: u64,   // 需要签文NFT数量
}

// ===== 商城配置账户 =====
#[account]
#[derive(InitSpace)]
pub struct ShopConfig {
    pub temple_config: Pubkey, // 关联的主配置账户

    // 商城物品配置
    #[max_len(10)]
    pub shop_items: Vec<ShopItem>,

    // 兑换比例配置
    pub incense_points_rate: u64, // 香火值兑换比例
    pub merit_rate: u64,          // 功德值兑换比例
}

impl ShopConfig {
    pub const SEED_PREFIX: &str = "config_shop";
}

// ===== 抽签配置账户 =====
#[account]
#[derive(InitSpace)]
pub struct FortuneConfigAccount {
    pub temple_config: Pubkey, // 关联的主配置账户

    // 签文配置
    pub fortune_config: FortuneConfig,        // 普通签概率配置
    pub buddha_fortune_config: FortuneConfig, // 佛签概率配置
}

impl FortuneConfigAccount {
    pub const SEED_PREFIX: &str = "config_fortune";
}

// ===== 奖励配置账户 =====
#[account]
#[derive(InitSpace)]
pub struct RewardConfig {
    pub temple_config: Pubkey, // 关联的主配置账户

    // 捐助等级配置
    #[max_len(5)]
    pub donation_levels: Vec<DonationLevelConfig>,

    // 捐助奖励配置
    #[max_len(5)]
    pub donation_rewards: Vec<DonationRewardConfig>,

    // 寺庙等级配置
    #[max_len(5)]
    pub level_configs: Vec<TempleLevelConfig>,
}

impl RewardConfig {
    pub const SEED_PREFIX: &str = "config_reward";
}

// 寺庙配置 - 主账户，负责配置和核心状态
#[account]
#[derive(InitSpace)]
pub struct TempleConfig {
    // 管理员配置
    pub owner: Pubkey,    // 寺庙管理员地址
    pub treasury: Pubkey, // 寺庙国库地址

    // 核心状态（需要签名权限的）
    pub level: u8,             // 当前寺庙等级（实时计算）
    pub created_at: i64,       // 创建时间
    pub total_buddha_nft: u32, // 佛像NFT数量（铸造权限）
    pub total_medal_nft: u32,  // 勋章NFT数量（铸造权限）
    pub total_amulets: u32,    // 御守数量（铸造权限）

    // 控制配置
    pub status: u8,     // 状态位控制 0则全部启用 其他值按位禁用对应的功能
    pub open_time: u64, // 上线时间戳

    // 配置账户引用
    pub shop_config: Pubkey,    // 商城配置账户
    pub fortune_config: Pubkey, // 抽签配置账户
    pub reward_config: Pubkey,  // 奖励配置账户
}

impl TempleConfig {
    pub const SEED_PREFIX: &str = "temple_v1";

    // 获取香型类型（从商城配置中查找）
    pub fn find_incense_type(shop_config: &ShopConfig, id: u8) -> Option<&ShopItem> {
        shop_config
            .shop_items
            .iter()
            .find(|item| item.id == id && matches!(item.item_type, ShopItemType::Incense))
    }

    // 获取香型价格
    pub fn get_fee_per_incense(shop_config: &ShopConfig, incense_id: u8) -> u64 {
        Self::find_incense_type(shop_config, incense_id)
            .map(|item| item.price)
            .unwrap_or(0)
    }

    // 注意：统计数据现在在GlobalStats中管理，此方法已废弃
    // 直接调用 global_stats.add_incense_value_and_merit()

    // 获取抽签概率配置
    pub fn get_fortune_config(
        fortune_config: &FortuneConfigAccount,
        has_buddha_nft: bool,
    ) -> &FortuneConfig {
        if has_buddha_nft {
            &fortune_config.buddha_fortune_config
        } else {
            &fortune_config.fortune_config
        }
    }

    // 获取捐助等级配置
    pub fn get_donation_level_config(
        reward_config: &RewardConfig,
        level: u8,
    ) -> Option<&DonationLevelConfig> {
        reward_config
            .donation_levels
            .iter()
            .find(|d| d.level == level)
    }

    // 检查香型是否存在
    pub fn is_incense_available(shop_config: &ShopConfig, incense_id: u8) -> bool {
        Self::find_incense_type(shop_config, incense_id).is_some()
    }

    // 动态计算等级
    pub fn calculate_temple_level(reward_config: &RewardConfig, global_stats: &GlobalStats) -> u8 {
        let incense_points = global_stats.total_incense_points;
        let donations_sol = global_stats.total_donations_sol();

        // 匹配等级要求
        for level_config in reward_config.level_configs.iter().rev() {
            if incense_points >= level_config.required_incense_points
                && global_stats.total_draw_fortune >= level_config.required_draw_fortune
                && global_stats.total_wishes >= level_config.required_wishes
                && donations_sol >= level_config.required_donations_sol
                && global_stats.total_fortune_nfts >= level_config.required_fortune_nfts
            {
                return level_config.level;
            }
        }

        1
    }

    // 更新寺庙等级
    pub fn update_level(&mut self, reward_config: &RewardConfig, global_stats: &GlobalStats) {
        self.level = Self::calculate_temple_level(reward_config, global_stats);
    }

    // 状态管理方法
    pub fn get_status_by_bit(&self, bit: TempleStatusBitIndex) -> bool {
        let status = 1u8 << (bit as u8);
        (self.status & status) == 0
    }

    pub fn set_status(&mut self, status: u8) {
        self.status = status;
    }

    pub fn set_status_by_bit(&mut self, bit: TempleStatusBitIndex, disabled: bool) {
        let mask = 1u8 << (bit as u8);
        if disabled {
            self.status |= mask; // 设置位为1（禁用）
        } else {
            self.status &= !mask; // 清除位为0（启用）
        }
    }

    // 是否可以进行操作 需要同时校验时间以及功能
    pub fn can_perform_operation(
        &self,
        bit: TempleStatusBitIndex,
        current_time: u64,
    ) -> Result<()> {
        // 上线时间
        if current_time < self.open_time {
            return err!(crate::error::ErrorCode::NotApproved);
        }

        // 功能状态
        if !self.get_status_by_bit(bit) {
            return err!(crate::error::ErrorCode::NotApproved);
        }

        Ok(())
    }
}
