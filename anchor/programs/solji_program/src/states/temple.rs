use anchor_lang::prelude::*;

use crate::global_error::GlobalError;

#[account]
#[derive(InitSpace)]
pub struct Temple {
    pub admin: Pubkey,
    // 寺庙等级
    pub level: u8,

    // 香火值
    pub total_incense_value: u64,

    // 功德值
    pub total_merit_value: u64,

    // 抽签次数
    pub total_lottery_count: u64,

    // 许愿次数
    pub total_wish_count: u64,

    // 捐助SOL
    pub total_donate_amount: u64,

    // 佛灯
    pub buddha_nft_count: u64,
}

impl Temple {
    pub fn new(admin: Pubkey) -> Self {
        Self {
            admin,
            level: 1,
            total_incense_value: 0,
            total_merit_value: 0,
            total_lottery_count: 0,
            total_wish_count: 0,
            total_donate_amount: 0,
            buddha_nft_count: 0,
        }
    }

    pub fn add_temple_incense_and_merit_attribute(
        &mut self,
        incense_value: u64,
        merit_value: u64,
    ) -> Result<()> {
        self.total_incense_value = self
            .total_incense_value
            .checked_add(incense_value)
            .ok_or(error!(GlobalError::MathOverflow))?;

        self.total_merit_value = self
            .total_merit_value
            .checked_add(merit_value)
            .ok_or(error!(GlobalError::MathOverflow))?;

        self.check_and_temple_upgrade()?;
        Ok(())
    }

    pub fn add_temple_lottery(&mut self) -> Result<()> {
        self.total_lottery_count = self
            .total_lottery_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.check_and_temple_upgrade()?;
        Ok(())
    }

    pub fn add_temple_wish(&mut self) -> Result<()> {
        self.total_wish_count = self
            .total_wish_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;
        self.check_and_temple_upgrade()?;
        Ok(())
    }

    pub fn add_temple_donate_amount(&mut self, amount: u64) -> Result<()> {
        self.total_donate_amount = self
            .total_donate_amount
            .checked_add(amount)
            .ok_or(GlobalError::MathOverflow)?;
        self.check_and_temple_upgrade()?;
        Ok(())
    }
    pub fn check_and_temple_upgrade(&mut self) -> Result<()> {
        // 达到最高级后不再读取下一条件
        if (self.level as usize) >= UPGRADE_CONDITIONS.len() - 1 {
            return Ok(());
        }
        let (incense, lottery, wish, donate) = UPGRADE_CONDITIONS[self.level as usize];
        if self.total_incense_value >= incense
            && self.total_lottery_count >= lottery
            && self.total_wish_count >= wish
            && self.total_donate_amount >= donate
        {
            self.level = self.level.checked_add(1).ok_or(GlobalError::MathOverflow)?;
        }
        Ok(())
    }
}

// 寺庙升级条件 , 香火值,抽签,许愿次数,捐助sol
pub const UPGRADE_CONDITIONS: [(u64, u64, u64, u64); 4] = [
    (0, 0, 0, 0),
    (10000, 5000, 3000, 100),
    (20000, 1000, 6000, 200),
    (30000, 2000, 8000, 300),
];
