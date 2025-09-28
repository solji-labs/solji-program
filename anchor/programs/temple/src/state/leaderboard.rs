use anchor_lang::prelude::*;

// 排行榜用户条目
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct LeaderboardUser {
    pub user: Pubkey,
    pub value: u64,
}

// 排行榜条目账户（存储详细分数）
#[account]
#[derive(InitSpace)]
pub struct LeaderboardEntry {
    pub bump: u8,
    pub user: Pubkey,              // 用户地址
    pub period: LeaderboardPeriod, // 排行榜周期
    pub incense_count: u64,        // 烧香次数
    pub incense_value: u64,        // 香火值
    pub merit: u64,                // 功德值
    pub last_updated: i64,         // 最后更新时间
}

impl LeaderboardEntry {
    pub const SEED_PREFIX: &'static str = "leaderboard_entry";

    pub fn new(user: Pubkey, period: LeaderboardPeriod, bump: u8) -> Self {
        let now = Clock::get().unwrap().unix_timestamp;
        Self {
            bump,
            user,
            period,
            incense_count: 0,
            incense_value: 0,
            merit: 0,
            last_updated: now,
        }
    }

    pub fn update(&mut self, incense_count: u64, incense_value: u64, merit: u64) {
        self.incense_count = self.incense_count.saturating_add(incense_count);
        self.incense_value = self.incense_value.saturating_add(incense_value);
        self.merit = self.merit.saturating_add(merit);
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    pub fn reset(&mut self) {
        self.incense_count = 0;
        self.incense_value = 0;
        self.merit = 0;
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }
}

// 主排行榜账户（存储排序后的用户地址和值列表）
#[account]
#[derive(InitSpace)]
pub struct Leaderboard {
    pub bump: u8,
    pub total_users: u32,        // 总用户数
    pub last_daily_reset: i64,   // 每日重置时间
    pub last_weekly_reset: i64,  // 每周重置时间
    pub last_monthly_reset: i64, // 每月重置时间
    // 存储前10名用户地址和值（已排序）
    #[max_len(10)]
    pub daily_users: Vec<LeaderboardUser>, // 每日活跃用户列表
    #[max_len(10)]
    pub weekly_users: Vec<LeaderboardUser>, // 每周活跃用户列表
    #[max_len(10)]
    pub monthly_users: Vec<LeaderboardUser>, // 每月活跃用户列表
}

impl Leaderboard {
    pub const SEED_PREFIX: &'static str = "leaderboard";

    // 初始化排行榜
    pub fn initialize(&mut self, bump: u8) {
        self.bump = bump;
        self.total_users = 0;
        self.last_daily_reset = Clock::get().unwrap().unix_timestamp;
        self.last_weekly_reset = self.last_daily_reset;
        self.last_monthly_reset = self.last_daily_reset;
    }

    // 检查是否需要重置排行榜周期
    pub fn check_and_reset_periods(&mut self) {
        let now = Clock::get().unwrap().unix_timestamp;

        // 每日重置 (24小时)
        if now - self.last_daily_reset >= 24 * 60 * 60 {
            self.last_daily_reset = now;
            self.daily_users.clear();
        }

        // 每周重置 (7天)
        if now - self.last_weekly_reset >= 7 * 24 * 60 * 60 {
            self.last_weekly_reset = now;
            self.weekly_users.clear();
        }

        // 每月重置 (30天)
        if now - self.last_monthly_reset >= 30 * 24 * 60 * 60 {
            self.last_monthly_reset = now;
            self.monthly_users.clear();
        }
    }

    // 更新用户在排行榜中的位置
    pub fn update_user_ranking(&mut self, user: Pubkey, value: u64, period: LeaderboardPeriod) {
        let user_list = match period {
            LeaderboardPeriod::Daily => &mut self.daily_users,
            LeaderboardPeriod::Weekly => &mut self.weekly_users,
            LeaderboardPeriod::Monthly => &mut self.monthly_users,
        };

        // 移除现有条目
        user_list.retain(|u| u.user != user);

        // 插入新条目
        user_list.push(LeaderboardUser { user, value });

        // 排序（降序）
        user_list.sort_by(|a, b| b.value.cmp(&a.value));

        // 保持前10
        if user_list.len() > 10 {
            user_list.truncate(10);
        }
    }

    // 获取用户排名（返回排名位置，0-based）
    pub fn get_incense_leaderboard(&self, user: &Pubkey, period: LeaderboardPeriod) -> Option<u32> {
        let user_list = match period {
            LeaderboardPeriod::Daily => &self.daily_users,
            LeaderboardPeriod::Weekly => &self.weekly_users,
            LeaderboardPeriod::Monthly => &self.monthly_users,
        };

        user_list
            .iter()
            .position(|u| u.user == *user)
            .map(|pos| pos as u32)
    }

    // 检查用户是否有视觉特效奖励（前3名）
    pub fn has_visual_effect(&self, user: &Pubkey, period: LeaderboardPeriod) -> bool {
        if let Some(rank) = self.get_incense_leaderboard(user, period) {
            rank <= 3
        } else {
            false
        }
    }
}

// 排行榜周期枚举
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, Debug)]
pub enum LeaderboardPeriod {
    Daily,
    Weekly,
    Monthly,
}
