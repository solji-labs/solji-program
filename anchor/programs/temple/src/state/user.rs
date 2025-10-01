use anchor_lang::prelude::*;

/// 用户个人状态
/// 存储单个用户的所有行为数据和限制
#[account]
#[derive(Debug, InitSpace)]
pub struct UserState {
    /// 用户钱包地址
    pub user: Pubkey,

    /// 用户累积功德值，可用于兑换额外操作
    pub karma_points: u64,

    /// 用户贡献的香火值总和
    pub total_incense_value: u64,

    /// 用户总消费金额 (lamports)
    pub total_sol_spent: u64,

    /// 用户总捐助金额 (lamports)
    pub total_donated: u64,

    /// 通过捐助解锁的额外烧香次数（每日重置）
    pub donation_unlocked_burns: u8,

    /// 今日已进行烧香操作次数 (每日重置)
    pub daily_burn_operations: u8,

    /// 今日已抽签次数 (每日重置)
    pub daily_draw_count: u8,

    /// 今日已许愿次数 (每日重置，为未来功能预留)
    pub daily_wish_count: u8,

    /// 上次操作日期，用于每日重置判断
    pub last_action_day: u16, // 存储自纪元开始的天数

    /// 总烧香操作次数统计
    pub total_burn_operations: u32,

    /// 总烧毁香的根数统计
    pub total_incense_burned: u32,

    /// 总抽签次数统计
    pub total_draw_count: u32,

    /// 总许愿次数统计 (为未来功能预留)
    pub total_wish_count: u32,

    /// 用户创建时间戳
    pub created_at: i64,

    /// 最后活跃时间戳
    pub last_active_at: i64,
}

impl UserState {
    /// PDA种子前缀
    pub const SEED_PREFIX: &'static str = "user_state_v1";

    /// 每日基础烧香次数限制
    pub const DAILY_BURN_LIMIT: u8 = 10;

    /// 每日基础抽签次数限制
    pub const DAILY_DRAW_LIMIT: u8 = 1;

    /// 每日基础许愿次数限制
    pub const DAILY_WISH_LIMIT: u8 = 3;

    /// 初始化用户状态
    pub fn initialize(&mut self, user: Pubkey, current_timestamp: i64) -> Result<()> {
        let current_day = (current_timestamp / 86400) as u16;

        self.user = user;
        self.karma_points = 0;
        self.total_incense_value = 0;
        self.total_sol_spent = 0;
        self.total_donated = 0;
        self.donation_unlocked_burns = 0;
        self.daily_burn_operations = 0;
        self.daily_draw_count = 0;
        self.daily_wish_count = 0;
        self.last_action_day = current_day;
        self.total_burn_operations = 0;
        self.total_incense_burned = 0;
        self.total_draw_count = 0;
        self.total_wish_count = 0;
        self.created_at = current_timestamp;
        self.last_active_at = current_timestamp;

        msg!("User state initialized for: {}", user);
        Ok(())
    }

    /// 检查并重置每日限制
    pub fn check_and_reset_daily_limits(&mut self) -> Result<()> {
        let current_day = (Clock::get()?.unix_timestamp / 86400) as u16;

        if self.last_action_day != current_day {
            self.daily_burn_operations = 0;
            self.daily_draw_count = 0;
            self.daily_wish_count = 0;
            self.donation_unlocked_burns = 0;
            self.last_action_day = current_day;
        }

        msg!("Daily limits checked and reset for: {}", self.user);
        Ok(())
    }

    /// 获取当前可用的烧香次数
    pub fn get_available_burn_operations(&self) -> u8 {
        let base_limit = Self::DAILY_BURN_LIMIT;
        // saturating_add: 先加上donation_unlocked_burns，如果结果大于u8::MAX，返回u8::MAX
        let total_limit = base_limit.saturating_add(self.donation_unlocked_burns);
        // saturating_sub: 先减去daily_burn_operations，如果结果小于0，返回0
        total_limit.saturating_sub(self.daily_burn_operations)
    }

    /// 获取当日可用抽签次数
    pub fn get_available_draw_count(&self) -> u8 {
        Self::DAILY_DRAW_LIMIT.saturating_sub(self.daily_draw_count)
    }

    /// 获取当日可用许愿次数
    pub fn get_available_wish_count(&self) -> u8 {
        Self::DAILY_WISH_LIMIT.saturating_sub(self.daily_wish_count)
    }

    /// 增加功德值
    pub fn add_karma_points(&mut self, amount: u64) -> Result<()> {
        self.karma_points = self
            .karma_points
            .checked_add(amount)
            .ok_or(UserError::KarmaPointsOverflow)?;

        self.last_active_at = Clock::get()?.unix_timestamp;

        Ok(())
    }

    /// 记录消费
    pub fn record_spending(&mut self, amount: u64) -> Result<()> {
        self.total_sol_spent = self
            .total_sol_spent
            .checked_add(amount)
            .ok_or(UserError::SpendingOverflow)?;
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 记录捐助
    pub fn record_donation(&mut self, amount: u64) -> Result<()> {
        self.total_donated = self
            .total_donated
            .checked_add(amount)
            .ok_or(UserError::DonationOverflow)?;
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 增加捐助解锁的烧香次数
    pub fn add_donation_unlocked_burns(&mut self, count: u8) -> Result<()> {
        self.donation_unlocked_burns = self.donation_unlocked_burns.saturating_add(count);
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 记录烧香操作
    pub fn record_burn_operation(&mut self, incense_count: u8) -> Result<()> {
        require!(
            self.get_available_burn_operations() > 0,
            UserError::DailyBurnLimitExceeded
        );

        self.daily_burn_operations = self.daily_burn_operations.saturating_add(1);
        self.total_burn_operations = self.total_burn_operations.saturating_add(1);
        self.total_incense_burned = self
            .total_incense_burned
            .saturating_add(incense_count as u32);
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 记录抽签操作
    pub fn record_draw_operation(&mut self) -> Result<()> {
        require!(
            self.get_available_draw_count() > 0,
            UserError::DailyDrawLimitExceeded
        );

        self.daily_draw_count = self.daily_draw_count.saturating_add(1);
        self.total_draw_count = self.total_draw_count.saturating_add(1);
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 记录许愿操作
    pub fn record_wish_operation(&mut self) -> Result<()> {
        require!(
            self.get_available_wish_count() > 0,
            UserError::DailyWishLimitExceeded
        );

        self.daily_wish_count = self.daily_wish_count.saturating_add(1);
        self.total_wish_count = self.total_wish_count.saturating_add(1);
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

/// 用户相关错误定义
#[error_code]
pub enum UserError {
    #[msg("Karma points overflow")]
    KarmaPointsOverflow,

    #[msg("Insufficient karma points")]
    InsufficientKarmaPoints,

    #[msg("Incense value overflow")]
    IncenseValueOverflow,

    #[msg("Spending amount overflow")]
    SpendingOverflow,

    #[msg("Donation amount overflow")]
    DonationOverflow,

    #[msg("Daily burn operation limit exceeded")]
    DailyBurnLimitExceeded,

    #[msg("Daily draw limit exceeded")]
    DailyDrawLimitExceeded,

    #[msg("Daily wish limit exceeded")]
    DailyWishLimitExceeded,

    #[msg("User state already exists")]
    UserStateAlreadyExists,

    #[msg("User state not found")]
    UserStateNotFound,

    #[msg("Unauthorized user access")]
    UnauthorizedUserAccess,
}
