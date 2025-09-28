use crate::error::ErrorCode;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

// 用户称号枚举
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum UserTitle {
    Pilgrim,   // 香客
    Disciple,  // 居士
    Protector, // 护法
    Patron,    // 供奉
    Abbot,     // 寺主
}

/// 根据功德值计算用户称号
fn calculate_title_from_merit(merit: u64) -> UserTitle {
    if merit >= 100000 {
        UserTitle::Abbot // 寺主
    } else if merit >= 10000 {
        UserTitle::Patron // 供奉
    } else if merit >= 1000 {
        UserTitle::Protector // 护法
    } else if merit >= 100 {
        UserTitle::Disciple // 居士
    } else {
        UserTitle::Pilgrim // 香客
    }
}

// 定义香型余额结构
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct IncenseBalance {
    pub incense_id: u8,
    pub balance: u64,
}

// 定义每日烧香次数结构
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DailyIncenseCount {
    pub incense_id: u8,
    pub count: u8,
}

// ==== 账户结构定义结束 ============

// 主用户状态账户
#[account]
#[derive(InitSpace)]
pub struct UserState {
    pub user: Pubkey,         // 用户地址
    pub has_buddha_nft: bool, // 是否拥有佛像NFT
    pub has_medal_nft: bool,  // 是否拥有勋章NFT
    pub bump: u8,

    // 随机数请求相关
    pub pending_random_request_id: Option<[u8; 32]>, // 待处理的随机数请求ID

    // 御守相关
    pub pending_amulets: u32, // 御守余额：铸造使用
}

impl UserState {
    pub const SEED_PREFIX: &str = "user_state";
}

// ===== 拆分后的子账户 =====

// 香火
#[account]
#[derive(InitSpace)]
pub struct UserIncenseState {
    pub user: Pubkey,
    pub title: UserTitle,    // 用户称号（基于功德值）
    pub incense_points: u64, // 香火值
    pub merit: u64,          // 功德值
    pub incense_number: u8,  // 每日烧香量
    pub update_time: i64,    // 更新时间
    pub bump: u8,

    // 香火余额和每日计数
    pub incense_balance: [IncenseBalance; 6],
    pub daily_incense_count: [DailyIncenseCount; 6],

    // 抽签相关
    pub daily_draw_count: u8,
    pub last_draw_time: i64,
    pub total_draws: u32, // 总抽签次数

    // 许愿相关
    pub daily_wish_count: u8,
    pub last_wish_time: i64,
    pub total_wishes: u32, // 总许愿次数
}

// 捐助
#[account]
#[derive(InitSpace)]
pub struct UserDonationState {
    pub user: Pubkey,
    pub donation_amount: u64,
    pub donation_level: u8,
    pub total_donation_count: u32,
    pub last_donation_time: i64,
    pub bump: u8,
}

impl UserIncenseState {
    pub const SEED_PREFIX: &str = "user_incense";

    /// 获取指定香型的余额
    pub fn get_incense_balance(&self, incense_id: u8) -> u64 {
        self.incense_balance
            .iter()
            .find(|item| item.incense_id == incense_id)
            .map(|item| item.balance)
            .unwrap_or(0)
    }

    /// 设置指定香型的余额
    pub fn set_incense_balance(&mut self, incense_id: u8, balance: u64) {
        if let Some(item) = self
            .incense_balance
            .iter_mut()
            .find(|item| item.incense_id == incense_id)
        {
            item.balance = balance;
        } else {
            // 查找空位置或替换第一个空记录
            for item in self.incense_balance.iter_mut() {
                if item.incense_id == 0 || item.incense_id == incense_id {
                    item.incense_id = incense_id;
                    item.balance = balance;
                    break;
                }
            }
        }
    }

    /// 增加指定香型的余额
    pub fn add_incense_balance(&mut self, incense_id: u8, amount: u64) {
        let current_balance = self.get_incense_balance(incense_id);
        self.set_incense_balance(incense_id, current_balance.saturating_add(amount));
    }

    /// 减少指定香型的余额
    pub fn subtract_incense_balance(&mut self, incense_id: u8, amount: u64) -> Result<()> {
        let current_balance: u64 = self.get_incense_balance(incense_id);
        if current_balance < amount {
            return err!(ErrorCode::InsufficientIncenseBalance);
        }
        self.set_incense_balance(incense_id, current_balance - amount);
        Ok(())
    }

    /// 获取指定香型的当日烧香次数
    pub fn get_daily_incense_count(&self, incense_id: u8) -> u8 {
        self.daily_incense_count
            .iter()
            .find(|item| item.incense_id == incense_id)
            .map(|item| item.count)
            .unwrap_or(0)
    }

    /// 设置指定香型的当日烧香次数
    pub fn set_daily_incense_count(&mut self, incense_id: u8, count: u8) {
        if let Some(item) = self
            .daily_incense_count
            .iter_mut()
            .find(|item| item.incense_id == incense_id)
        {
            item.count = count;
        } else {
            // 查找空位置或替换第一个空记录
            for item in self.daily_incense_count.iter_mut() {
                if item.incense_id == 0 || item.incense_id == incense_id {
                    item.incense_id = incense_id;
                    item.count = count;
                    break;
                }
            }
        }
    }

    /// 检查香火量是否超出限制
    pub fn check_daily_incense_limit(&self, incense_id: u8, amount: u8) -> Result<()> {
        // 1. 先判断是否跨天，跨天则重置该香型次数
        let now = Clock::get()?.unix_timestamp;
        let is_new_day = now - self.update_time >= 86400;
        let current_count = if is_new_day {
            0
        } else {
            self.get_daily_incense_count(incense_id)
        };

        // 2. 校验次数（单个香型每日≤10）
        if current_count + amount > 10 {
            return err!(ErrorCode::ExceedDailyIncenseLimit);
        }
        Ok(())
    }

    /// 更新当日烧香次数
    pub fn update_daily_count(&mut self, incense_id: u8, amount: u8) {
        let now = Clock::get().unwrap().unix_timestamp;
        // 跨天则重置所有次数+更新重置时间
        let is_new_day = now - self.update_time >= 86400;
        if is_new_day {
            // 手动重置固定大小数组
            for item in self.daily_incense_count.iter_mut() {
                item.incense_id = 0;
                item.count = 0;
            }
            self.incense_number = 0;
            self.update_time = now;
        }
        // 累加当前香型次数
        self.incense_number += amount;
        let current_count = self.get_daily_incense_count(incense_id);
        self.set_daily_incense_count(incense_id, current_count + amount);
    }

    // 增加用户的香火值和功德值，并自动更新称号
    pub fn add_incense_value_and_merit(&mut self, incense_value: u64, merit: u64) {
        self.incense_points = self
            .incense_points
            .checked_add(incense_value)
            .unwrap_or(self.incense_points);
        self.merit = self.merit.checked_add(merit).unwrap_or(self.merit);

        // 自动更新称号
        self.title = calculate_title_from_merit(self.merit);
    }

    /// 检查是否可以免费抽签
    pub fn can_draw_free(&self) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_draw_time >= 86400; // 24小时

        if is_new_day {
            // 新的一天，重置计数
            true
        } else {
            // 同一天，检查免费次数
            self.daily_draw_count < 1
        }
    }

    /// 更新抽签计数
    pub fn update_draw_count(&mut self) {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_draw_time >= 86400;

        if is_new_day {
            // 新的一天，重置计数
            self.daily_draw_count = 1;
        } else {
            // 同一天，增加计数
            self.daily_draw_count += 1;
        }

        self.last_draw_time = now;
        self.total_draws = self.total_draws.saturating_add(1);
    }

    /// 消耗功德值进行额外抽签（使用动态配置的消耗值）
    pub fn consume_merit_for_draw(&mut self, merit_cost: u64) -> Result<()> {
        if self.merit < merit_cost {
            return err!(ErrorCode::InsufficientMerit);
        }
        self.merit -= merit_cost;
        Ok(())
    }

    /// 更新每日许愿次数
    pub fn update_wish_count(&mut self) {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_wish_time >= 86400;

        if is_new_day {
            // 新的一天，重置计数
            self.daily_wish_count = 1;
        } else {
            // 同一天，增加计数
            self.daily_wish_count += 1;
        }

        self.last_wish_time = now;
        self.total_wishes = self.total_wishes.saturating_add(1);
    }

    /// 许愿每日超过三次需要消耗功德
    pub fn consume_merit_for_wish(&mut self, merit_cost: u64) -> Result<()> {
        if self.merit < merit_cost {
            return err!(ErrorCode::InsufficientMerit);
        }
        self.merit -= merit_cost;
        Ok(())
    }

    /// 检查是否可以免费许愿
    pub fn can_wish_free(&self) -> bool {
        let now = Clock::get().unwrap().unix_timestamp;
        let is_new_day = now - self.last_wish_time >= 86400;

        if is_new_day {
            true // 新的一天可以免费许愿
        } else {
            self.daily_wish_count < 3 // 同一天检查次数
        }
    }
}

impl UserDonationState {
    pub const SEED_PREFIX: &str = "user_donation";

    /// 根据捐助金额计算等级
    pub fn calculate_donation_level(&self) -> u8 {
        let donation_sol = self.donation_amount as f64 / 1_000_000_000.0; // 转换为SOL

        if donation_sol >= 5.0 {
            4 // 至尊供奉
        } else if donation_sol >= 1.0 {
            3 // 金牌护法
        } else if donation_sol >= 0.2 {
            2 // 银牌居士
        } else if donation_sol >= 0.05 {
            1 // 铜牌信士
        } else {
            0 // 无等级
        }
    }

    /// 更新捐助等级
    pub fn update_donation_level(&mut self) {
        self.donation_level = self.calculate_donation_level();
    }

    /// 检查是否可以免费mint佛像 (>0.5 SOL)
    pub fn can_mint_buddha_free(&self) -> bool {
        let donation_sol = self.donation_amount as f64 / 1_000_000_000.0;
        donation_sol >= 0.5
    }

    /// 捐助等级获取奖励—— （功德奖励， 寺庙香火值）
    pub fn get_donation_rewards(&self) -> (u64, u64) {
        // (merit, incense_points)
        match self.donation_level {
            1 => (65, 1200),       // 铜牌信士
            2 => (1300, 6300),     // 银牌居士
            3 => (14000, 30000),   // 金牌护法
            4 => (120000, 100000), // 至尊供奉
            _ => (0, 0),           // 无等级
        }
    }

    /// 处理捐助逻辑
    pub fn process_donation(&mut self, amount_lamports: u64) {
        let now = Clock::get().unwrap().unix_timestamp;

        // 更新捐助金额
        self.donation_amount = self.donation_amount.saturating_add(amount_lamports);

        // 更新捐助统计
        self.total_donation_count = self.total_donation_count.saturating_add(1);
        self.last_donation_time = now;

        // 更新等级
        self.update_donation_level();
    }
}
