use anchor_lang::prelude::*;

use crate::{
    global_error::GlobalError,
    instructions::WishCode,
    states::{IncenseRule, IncenseType, MedalLevel},
};

#[account]
#[derive(InitSpace)]
pub struct UserInfo {
    pub user: Pubkey,
    pub level: u8,
    #[max_len(4)]
    pub burn_count: [u32; 6],
    pub total_burn_count: u64,
    pub incense_buy_count: [u32; 6],
    pub incense_donate_count: [u32; 6],
    pub merit_value: u64,
    pub incense_value: u64,
    pub incense_time: i64,
    pub donate_amount: u64,
    pub donate_count: u64,
    pub donate_merit_value: u64,
    pub donate_incense_value: u64,
    pub current_medal_level: MedalLevel,
    pub lottery_count: u32,
    pub lottery_is_free: bool,
    pub lottery_time: i64,
    pub wish_count: u32,
    pub wish_update_time: i64,
    pub wish_daily_count: u32,
    pub amulet_count: u64,
    pub has_sbt_token: bool,
    pub has_burn_token: [bool; 6],
    pub stake_count: u64,
    pub tower_level: i8,
}

#[account]
#[derive(InitSpace)]
pub struct UserStake {
    pub user: Pubkey,
    pub token: Pubkey,
    pub merit_value: u64,
    pub status: u8,
    pub stake_start_time: i64,
    pub requst_unstake_time: i64,
    pub unstake_end_time: i64,
}

impl UserStake {
    pub const SECONDS_PER_DAY: i64 = 86400;
    pub const SECONDS_PER_MINUTE: i64 = 60;
    pub const MINUTES_PER_DAY: i64 = 24 * 60;
    pub fn new(user: Pubkey, token: Pubkey, time: i64) -> Self {
        Self {
            user: user,
            token: token,
            merit_value: 0,
            status: 0,
            stake_start_time: time,
            requst_unstake_time: 0,
            unstake_end_time: 0,
        }
    }

    pub fn days_since_start_clamped(&self, time: i64) -> Result<i64> {
        let start = self.stake_start_time;
        let elapsed = time.saturating_sub(start);
        Ok(elapsed / Self::SECONDS_PER_DAY)
    }

    // pub fn days_since_start_clamped(&self, time: i64) -> Result<i64> {
    //     let diff_secs = time.saturating_sub(self.stake_start_time);
    //     if diff_secs == 0 {
    //         Ok(0)
    //     } else {
    //         let a = (diff_secs + Self::SECONDS_PER_MINUTE - 1) / Self::SECONDS_PER_MINUTE;
    //         Ok(a)
    //     }
    // }

    pub fn set_request_unstake(&mut self, time: i64) -> Result<()> {
        self.requst_unstake_time = time;
        self.status = 1;
        Ok(())
    }
    pub fn set_unstake_confirm(&mut self, time: i64) -> Result<()> {
        self.unstake_end_time = time;
        self.status = 2;
        Ok(())
    }

    pub fn check_request_unstake(&self, time: i64) -> Result<()> {
        if self.requst_unstake_time == 0 {
            return err!(GlobalError::UnstakeNotRequest);
        }
        let value = time
            .checked_sub(self.requst_unstake_time)
            .ok_or(GlobalError::MathUnderflow)?;
        if value / Self::MINUTES_PER_DAY < 2 {
            return err!(GlobalError::UnstakeTooSoon);
        }
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub enum ActivityEnum {
    Burn,
    Donate,
    Lottery,
    Wish,
    Like,
}

impl UserInfo {
    pub fn new(user: Pubkey) -> Self {
        Self {
            user,
            level: 0,
            burn_count: [0; 6],
            total_burn_count: 0,
            incense_buy_count: [0; 6],
            incense_donate_count: [0; 6],
            merit_value: 0,
            incense_value: 0,
            incense_time: 0,
            donate_amount: 0,
            donate_count: 0,
            donate_merit_value: 0,
            donate_incense_value: 0,
            current_medal_level: MedalLevel::None,
            lottery_count: 0,
            lottery_is_free: true,
            lottery_time: 0,
            wish_count: 0,
            wish_update_time: 0,
            wish_daily_count: 0,
            amulet_count: 0,
            has_sbt_token: false,
            has_burn_token: [false; 6],
            stake_count: 0,
            tower_level: -1,
        }
    }

    pub fn update_incense_donate_count(
        &mut self,
        incense_type: IncenseType,
        number: u64,
    ) -> Result<()> {
        self.incense_donate_count[incense_type as usize] = self.incense_donate_count
            [incense_type as usize]
            .checked_add(number as u32)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    pub fn update_incense_buy_count(
        &mut self,
        incense_type: IncenseType,
        number: u64,
    ) -> Result<()> {
        self.incense_buy_count[incense_type as usize] = self.incense_buy_count
            [incense_type as usize]
            .checked_add(number as u32)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    pub fn get_burn_count(&self, incense_type: IncenseType) -> u32 {
        self.burn_count[incense_type as usize]
    }

    pub fn update_user_info(
        &mut self,
        user: Pubkey,
        incense_type: IncenseType,
        incense_rule: IncenseRule,
    ) -> Result<()> {
        let now_ts = Clock::get().unwrap().unix_timestamp;
        self.user = user;

        if self.burn_count[incense_type as usize] >= 10
            || self.incense_buy_count[incense_type as usize] < 1
        {
            self.incense_donate_count[incense_type as usize] = self.incense_donate_count
                [incense_type as usize]
                .checked_sub(1)
                .ok_or(GlobalError::MathUnderflow)?;
        } else {
            self.incense_buy_count[incense_type as usize] = self.incense_buy_count
                [incense_type as usize]
                .checked_sub(1)
                .ok_or(GlobalError::MathUnderflow)?;
        }

        self.burn_count[incense_type as usize] = self.burn_count[incense_type as usize]
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.merit_value = self
            .merit_value
            .checked_add(incense_rule.merit_value)
            .ok_or(GlobalError::MathOverflow)?;

        self.incense_value = self
            .incense_value
            .checked_add(incense_rule.incense_value)
            .ok_or(GlobalError::MathOverflow)?;

        self.total_burn_count = self
            .total_burn_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.incense_time = now_ts;

        self.update_level()?;
        Ok(())
    }

    pub fn update_level(&mut self) -> Result<()> {
        let total_merit = self
            .merit_value
            .checked_add(self.donate_merit_value)
            .ok_or(GlobalError::MathOverflow)?;

        if total_merit < 5_000 {
            return Ok(());
        }

        let new_level = match total_merit {
            5_000..=29_999 => 1,
            30_000..=149_999 => 2,
            150_000..=499_999 => 3,
            _ => 4,
        };
        if self.level == new_level {
            return Ok(());
        }
        self.level = new_level;
        Ok(())
    }

    pub fn deduction(&mut self, value: u64) -> Result<()> {
        self.merit_value = self
            .merit_value
            .checked_sub(value)
            .ok_or(GlobalError::MathUnderflow)?;
        Ok(())
    }

    pub fn update_lottery_count(&mut self, now_ts: i64, value: u64) -> Result<()> {
        self.lottery_is_free = false;
        self.lottery_time = now_ts;

        self.lottery_count = self
            .lottery_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.merit_value = self
            .merit_value
            .checked_add(value)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    pub fn update_user_wish_count(&mut self) -> Result<()> {
        self.wish_count = self
            .wish_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.wish_daily_count = self
            .wish_daily_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.wish_update_time = Clock::get().unwrap().unix_timestamp;

        self.merit_value = self
            .merit_value
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    pub fn check_is_free(&mut self) -> Result<()> {
        let now_ts = Clock::get()?.unix_timestamp;
        let current_day = now_ts / 86_400;
        let last_day = self.wish_update_time / 86_400;

        if current_day > last_day {
            self.wish_daily_count = 0;
        }
        Ok(())
    }

    pub fn check_wish_daily_count(&mut self, value: u64) -> Result<()> {
        if self.wish_daily_count >= 3 {
            if self.merit_value < value {
                return err!(WishCode::Insufficient);
            }
            self.deduction(value)?;
        }
        Ok(())
    }

    pub fn update_user_donate_info(
        &mut self,
        donate_merit_value: u64,
        donate_incense_value: u64,
    ) -> Result<()> {
        if donate_merit_value > 0 {
            self.donate_merit_value = self
                .donate_merit_value
                .checked_add(donate_merit_value)
                .ok_or(GlobalError::MathOverflow)?;

            self.donate_incense_value = self
                .donate_incense_value
                .checked_add(donate_incense_value)
                .ok_or(GlobalError::MathOverflow)?;
        }
        self.update_level()?;
        Ok(())
    }

    pub fn update_user_donate_amount(&mut self, amount: u64) -> Result<()> {
        self.donate_amount = self
            .donate_amount
            .checked_add(amount)
            .ok_or(GlobalError::MathOverflow)?;
        self.donate_count = self
            .donate_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    pub fn amulet_increment(&mut self) -> Result<()> {
        self.amulet_count = self
            .amulet_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    pub fn stake_increment(&mut self) -> Result<()> {
        self.stake_count = self
            .stake_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    const BASE_DAILY_REWARD: u64 = 30;
    const SCALE: u64 = 100;
    pub fn get_calculate_reward(&self, day: i64) -> Result<u64> {
        let level_scaled: u64 = match self.current_medal_level {
            MedalLevel::Bronze => 100,  // 1.00 -> 100
            MedalLevel::Silver => 150,  // 1.50 -> 150
            MedalLevel::Gold => 250,    // 2.50 -> 250
            MedalLevel::Supreme => 400, // 4.00 -> 400
            _ => 100,
        };

        let time_scaled: u64 = if day == 7 {
            100 // 1.00
        } else if day >= 8 && day <= 30 {
            110 // 1.10
        } else {
            125 // 1.25
        };

        // reward = (BASE_DAILY_REWARD * level_mul * time_mul) / (SCALE * SCALE)
        let tmp = (Self::BASE_DAILY_REWARD as u128)
            .checked_mul(level_scaled as u128)
            .and_then(|v| v.checked_mul(time_scaled as u128))
            .ok_or(error!(GlobalError::MathOverflow))?;

        let denom = (Self::SCALE as u128)
            .checked_mul(Self::SCALE as u128)
            .unwrap(); // 100*100

        let reward = tmp / denom;

        Ok(reward as u64)
    }

    pub fn increment_stake_merit(&mut self, merit: u64) -> Result<()> {
        self.merit_value = self
            .merit_value
            .checked_add(merit)
            .ok_or(GlobalError::MathOverflow)?;
        self.update_level()?;
        Ok(())
    }
}
