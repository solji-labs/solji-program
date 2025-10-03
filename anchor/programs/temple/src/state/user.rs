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

    /// 总购买次数统计
    pub total_buy_count: u64,

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
        self.total_buy_count = 0;
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

    // 烧香
    pub fn burn_incense(
        &mut self,
        karma_points: u64,
        incense_value: u64,
        amount: u32,
    ) -> Result<()> {
        // 增加功德值
        self.karma_points = self
            .karma_points
            .checked_add(karma_points)
            .ok_or(UserError::KarmaPointsOverflow)?;

        // 增加香火值
        self.total_incense_value = self
            .total_incense_value
            .checked_add(incense_value)
            .ok_or(UserError::IncenseValueOverflow)?;

        // 增加烧香次数
        self.total_burn_operations = self
            .total_burn_operations
            .checked_add(1)
            .ok_or(UserError::BurnOperationsOverflow)?;

        // 增加烧香根数
        self.total_incense_burned = self
            .total_incense_burned
            .checked_add(amount)
            .ok_or(UserError::IncenseBurnedOverflow)?;

        // 增加今日烧香次数
        self.daily_burn_operations = self
            .daily_burn_operations
            .checked_add(1)
            .ok_or(UserError::DailyBurnLimitExceeded)?;

        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    // 获取功德值
    pub fn get_karma_points(&self) -> u64 {
        self.karma_points
    }

    // 获取今日抽签次数
    pub fn get_daily_draw_count(&self) -> u8 {
        self.daily_draw_count
    }

    /// 获取当日可用许愿次数
    pub fn get_available_wish_count(&self) -> u8 {
        Self::DAILY_WISH_LIMIT.saturating_sub(self.daily_wish_count)
    }

    /// 增加购买次数
    pub fn add_buy_count(&mut self) -> Result<()> {
        // saturating_add: 先加上1，如果结果大于u64::MAX，返回u64::MAX
        self.total_buy_count = self
            .total_buy_count
            .checked_add(1)
            .ok_or(UserError::BuyCountOverflow)?;
        Ok(())
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

    /// 抽签
    pub fn draw_fortune(&mut self, karma_points: u64) -> Result<()> {
        self.daily_draw_count = self.daily_draw_count.saturating_add(1);
        self.total_draw_count = self.total_draw_count.saturating_add(1);
        require!(
            self.karma_points >= karma_points,
            UserError::NotEnoughKarmaPoints
        );
        if karma_points > 0 {
            self.karma_points = self.karma_points.saturating_sub(karma_points);
        }
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

#[account]
#[derive(Debug, InitSpace)]
pub struct UserIncenseState {
    pub user: Pubkey,

    // 拥有的香
    pub incense_having_balances: [IncenseBalance; 6],

    // 已烧香的香
    pub incense_burned_balances: [IncenseBalance; 6],

    // total = having + burned
    pub incense_total_balances: [IncenseBalance; 6],

    // 最后活跃时间戳
    pub last_active_at: i64,
}

#[derive(Debug, AnchorDeserialize, AnchorSerialize, Clone, InitSpace)]
pub struct IncenseBalance {
    pub incense_type_id: u8,
    pub balance: u64,
}

impl UserIncenseState {
    /// PDA种子前缀
    pub const SEED_PREFIX: &'static str = "user_incense_state_v1";

    /// 获取拥有的香的余额
    pub fn get_incense_having_balance(&self, incense_type_id: u8) -> u64 {
        for balance in self.incense_having_balances.iter() {
            if balance.incense_type_id == incense_type_id {
                return balance.balance;
            }
        }
        0
    }

    /// 获取已烧香的香的余额
    pub fn get_incense_burned_balance(&self, incense_type_id: u8) -> u64 {
        for balance in self.incense_burned_balances.iter() {
            if balance.incense_type_id == incense_type_id {
                return balance.balance;
            }
        }
        0
    }

    /// 增加拥有的香的余额
    pub fn add_incense_balance(&mut self, incense_type_id: u8, amount: u64) -> Result<()> {
        for balance in self.incense_having_balances.iter_mut() {
            if balance.incense_type_id == incense_type_id {
                balance.balance = balance
                    .balance
                    .checked_add(amount)
                    .ok_or(UserError::IncenseValueOverflow)?;
            }
        }
        for balance in self.incense_total_balances.iter_mut() {
            if balance.incense_type_id == incense_type_id {
                balance.balance = balance
                    .balance
                    .checked_add(amount)
                    .ok_or(UserError::IncenseValueOverflow)?;
            }
        }
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }

    /// 减少拥有的香的余额
    pub fn sub_incense_balance(&mut self, incense_type_id: u8, amount: u64) -> Result<()> {
        // 检查拥有的香是否足够
        let having_balance = self.get_incense_having_balance(incense_type_id);
        require!(having_balance >= amount, UserError::InsufficientKarmaPoints);

        // 减少拥有的香的余额
        for balance in self.incense_having_balances.iter_mut() {
            if balance.incense_type_id == incense_type_id {
                balance.balance = balance
                    .balance
                    .checked_sub(amount)
                    .ok_or(UserError::IncenseValueOverflow)?;
            }
        }

        // 增加已烧香的香的余额
        for balance in self.incense_burned_balances.iter_mut() {
            if balance.incense_type_id == incense_type_id {
                balance.balance = balance
                    .balance
                    .checked_add(amount)
                    .ok_or(UserError::IncenseValueOverflow)?;
            }
        }

        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }
}

/// 用户相关错误定义
#[error_code]
pub enum UserError {
    #[msg("Invalid randomness account")]
    InvalidRandomnessAccount,
    #[msg("Not enough karma points")]
    NotEnoughKarmaPoints,
    #[msg("Burn operations overflow")]
    BurnOperationsOverflow,
    #[msg("Incense burned overflow")]
    IncenseBurnedOverflow,
    #[msg("Buy count overflow")]
    BuyCountOverflow,

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
