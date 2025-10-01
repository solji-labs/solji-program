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
    #[max_len(4)]
    pub burn_count: [u32; 6],
    pub incense_property_count: [u32; 6],
    pub merit_value: u64,
    pub incense_value: u64,
    pub incense_time: i64,
    pub donate_amount: u64,
    pub donate_merit_value: u64,
    pub donate_incense_value: u64,
    pub current_medal_level: Option<MedalLevel>,
    pub lottery_count: u32,
    pub lottery_is_free: bool,
    pub lottery_time: i64,
    pub wish_total_count: u32,
    pub wish_update_time: i64,
    pub wish_daily_count: u32,
    pub has_sbt_token: bool,
}

impl UserInfo {
    pub fn new(user: Pubkey) -> Self {
        Self {
            user,
            burn_count: [0; 6],
            incense_property_count: [0; 6],
            merit_value: 0,
            incense_value: 0,
            incense_time: 0,
            donate_amount: 0,
            donate_merit_value: 0,
            donate_incense_value: 0,
            current_medal_level: Some(MedalLevel::None),
            lottery_count: 0,
            lottery_is_free: true,
            lottery_time: 0,
            wish_total_count: 0,
            wish_update_time: 0,
            wish_daily_count: 0,
            has_sbt_token: false,
        }
    }

    pub fn update_incense_property_count(
        &mut self,
        incense_type: IncenseType,
        number: u64,
    ) -> Result<()> {
        self.incense_property_count[incense_type as usize] = self.incense_property_count
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

        if self.burn_count[incense_type as usize] >= 10 {
            self.incense_property_count[incense_type as usize] = self.incense_property_count
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

        self.incense_time = now_ts;
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
        self.wish_total_count = self
            .wish_total_count
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

    pub fn check_is_free(&mut self) {
        let last_day = (self.wish_update_time + 8 * 3600) / 86400;
        let now_ts = Clock::get().unwrap().unix_timestamp;
        let current_day = (now_ts + 8 * 3600) / 86400;
        if current_day > last_day {
            self.wish_daily_count = 0;
        }
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
        Ok(())
    }

    pub fn update_user_donate_amount(&mut self, amount: u64) -> Result<()> {
        self.donate_amount = self
            .donate_amount
            .checked_add(amount)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }
}
