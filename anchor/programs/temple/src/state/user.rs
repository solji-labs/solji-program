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
 

    /// 通过捐助解锁的额外烧香次数（每日重置）
    pub donation_unlocked_burns: u8,

    /// 今日已进行烧香操作次数 (每日重置)
    pub daily_burn_count: u8,

    /// 今日已抽签次数 (每日重置)
    pub daily_draw_count: u8,

    /// 今日已许愿次数 (每日重置，为未来功能预留)
    pub daily_wish_count: u8,

    /// 上次操作日期，用于每日重置判断
    pub last_action_day: u16, // 存储自纪元开始的天数
 

    /// 总烧香操作次数统计
    pub total_burn_count: u32,

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

    /// 每日免费许愿次数限制
    pub const DAILY_FREE_WISH_LIMIT: u8 = 3;

    /// 初始化用户状态
    pub fn initialize(&mut self, user: Pubkey, current_timestamp: i64) -> Result<()> {
        let current_day = (current_timestamp / 86400) as u16;

        self.user = user;
        self.karma_points = 0;
        self.total_incense_value = 0;
        self.total_sol_spent = 0; 
        self.donation_unlocked_burns = 0;
        self.daily_burn_count = 0;
        self.daily_draw_count = 0;
        self.daily_wish_count = 0;
        self.last_action_day = current_day; 
        self.total_burn_count = 0; 
        self.total_draw_count = 0;
        self.total_wish_count = 0;
        self.created_at = current_timestamp;
        self.last_active_at = current_timestamp;

        msg!("User state initialized for: {}", user);
        Ok(())
    }

    pub fn donate_fund(&mut self, amount: u64, current_timestamp: i64) -> Result<()> {
        self.check_and_reset_daily_limits()?;
        // 每捐助0.01sol ，可以增加烧香的1次
        // 如果捐助 0.011sol，可以增加烧香的1次, 0.009sol 不增加
        let donate_sol = (amount / 100_000_000) as f64;

        //floor: 向下取整 - 0.011 -> 0.01； 0.009 -> 0.00
        let donate_burns =  if donate_sol >= 1.0 {
            donate_sol.floor() as u8
        } else {
            0
        };
        
        //增加烧香次数
        self.donation_unlocked_burns = self.donation_unlocked_burns.saturating_add(donate_burns);
        
        //增加功德值
        let add_karma_points  = if donate_sol >=5.0 {
            1200000
        } else if donate_sol >=1.0 {
            140000
        } else if donate_sol >=0.2 {
            1300
        } else {
            65
        };
        self.karma_points = self.karma_points.saturating_add(add_karma_points);
        
        self.last_active_at = current_timestamp;
        Ok(())
    }

    /// 检查并重置每日限制
    pub fn check_and_reset_daily_limits(&mut self) -> Result<()> {
        let current_day = (Clock::get()?.unix_timestamp / 86400) as u16;

        if self.last_action_day != current_day {
            self.daily_burn_count = 0;
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
        // saturating_sub: 先减去daily_burn_count，如果结果小于0，返回0
        total_limit.saturating_sub(self.daily_burn_count)
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
        self.total_burn_count = self
            .total_burn_count
            .checked_add(1)
            .ok_or(UserError::BurnCountOverflow)?;

        // 增加今日烧香次数
        self.daily_burn_count = self
            .daily_burn_count
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
    pub fn get_daily_wish_count(&self) -> u8 {
        self.daily_wish_count
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

        self.daily_burn_count = self.daily_burn_count.saturating_add(1);
        self.total_burn_count = self.total_burn_count.saturating_add(1);
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

    /// 许愿
    pub fn create_wish(&mut self, karma_points: u64) -> Result<()> {
        if karma_points > 0 {
            require!(
                self.karma_points >= karma_points,
                UserError::NotEnoughKarmaPoints
            );
            self.karma_points = self.karma_points.saturating_sub(karma_points);
        }
        self.daily_wish_count = self.daily_wish_count.saturating_add(1);
        self.total_wish_count = self.total_wish_count.saturating_add(1);
        self.last_active_at = Clock::get()?.unix_timestamp;
        Ok(())
    }


    pub fn get_total_wish_count(&self) -> u32 {
        self.total_wish_count.into()
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




    pub fn initialize(&mut self, user: Pubkey,current_timestamp: i64) -> Result<()> {
        self.user = user;
        for i in 0..6 {
            self.incense_having_balances[i] = IncenseBalance {
                incense_type_id: (i + 1) as u8,
                balance: 0,
            };
            self.incense_burned_balances[i] = IncenseBalance {
                incense_type_id: (i + 1) as u8,
                balance: 0,
            };
            self.incense_total_balances[i] = IncenseBalance {
                incense_type_id: (i + 1) as u8,
                balance: 0,
            };
        }
        self.last_active_at = current_timestamp;
        Ok(())
    }


    pub fn airdrop_incense_by_donation(&mut self, amount: u64) -> Result<()> {
        let (incense_type_id, incense_amount) = if amount >= 5_000_000_000 && amount < 50_000_000_000 {
            (5, 10)
        } else {
            (6, 5)
        };

        self.add_incense_balance(incense_type_id, incense_amount)?;

        Ok(())
    }



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


#[account]
#[derive(Debug, InitSpace)]
pub struct UserDonationState {
    pub user: Pubkey,
    pub total_donation_amount: u64,
    pub total_donation_count: u64,
    pub donation_level: u8,
    pub last_donation_at: i64,
    
    pub can_mint_buddha_nft: bool,
    pub has_minted_buddha_nft: bool,
}

impl UserDonationState {
 pub const SEED_PREFIX: &'static str = "user_donation_state_v1";

    pub fn initialize(&mut self, user: Pubkey) -> Result<()> {
        self.user = user;
        self.can_mint_buddha_nft = false;
        self.has_minted_buddha_nft = false;
        self.total_donation_amount = 0;
        self.total_donation_count = 0;
        self.donation_level = self.calculate_donation_level();
        self.last_donation_at = 0;
        Ok(())
    }

    pub fn can_mint_buddha_nft(&self) -> bool {
        self.can_mint_buddha_nft
    }   
    
    pub fn has_minted_buddha_nft(&self) -> bool {
        self.has_minted_buddha_nft   
    }   
    
    pub fn mint_buddha_nft(&mut self) -> Result<()> {
        self.has_minted_buddha_nft = true;
        Ok(())
    }

    // donate fund
    pub fn donate_fund(&mut self, amount: u64, current_timestamp: i64) -> Result<u64> {
        self.total_donation_amount = self.total_donation_amount.checked_add(amount).unwrap();
        self.total_donation_count = self.total_donation_count.checked_add(1).unwrap();
        self.donation_level = self.calculate_donation_level();
        self.last_donation_at = current_timestamp;

        // more than 0.5 SOL can mint
        // 1 sol = 1_000_000_000 lamports
        if self.total_donation_amount >= 500_000_000 {
            self.can_mint_buddha_nft = true;
        }

        Ok(self.total_donation_amount)
    }

    // calculate donation level
    pub fn calculate_donation_level(&mut self) -> u8 {
        let donation_sol = (self.total_donation_amount as f64 / 1_000_000_000.0);

        if donation_sol >= 5.0 {
            4 // Supreme Patron
        } else if donation_sol >= 1.0 {
            3 // Gold Protector
        } else if donation_sol >= 0.2 {
            2 // Silver Disciple
        } else if donation_sol >= 0.05 {
            1 // Bronze Believer
        } else {
            0 // No level
        }
    }
     
 
}


/// 用户相关错误定义
#[error_code]
pub enum UserError {
    #[msg("Invalid user")]
    InvalidUser,
    #[msg("Burn count overflow")]
    BurnCountOverflow,
    #[msg("Wish count overflow")]
    WishCountOverflow,
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
