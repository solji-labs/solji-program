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

    pub total_burn_count: u64,

    // 抽签次数
    pub total_lottery_count: u64,

    // 许愿次数
    pub total_wish_count: u64,

    // 捐助SOL
    pub total_donate_amount: u64,

    pub total_donate_count: u64,

    // 护身符
    pub total_amulet_count: u64,

    // 佛灯
    pub buddha_nft_count: u64,

    pub wealth: u64,
}

impl Temple {
    pub fn new(admin: Pubkey) -> Self {
        Self {
            admin,
            level: 1,
            total_incense_value: 0,
            total_merit_value: 0,
            total_burn_count: 0,
            total_lottery_count: 0,
            total_wish_count: 0,
            total_donate_amount: 0,
            total_donate_count: 0,
            total_amulet_count: 0,
            buddha_nft_count: 0,
            wealth: 0,
        }
    }

    pub fn add_temple_incense_and_merit_attribute_upgrade(
        &mut self,
        incense_value: u64,
        merit_value: u64,
    ) -> Result<()> {
        self.add_temple_incense_and_merit(incense_value, merit_value)?;

        self.total_burn_count = self
            .total_burn_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;

        self.check_and_temple_upgrade()?;
        Ok(())
    }

    pub fn add_temple_incense_and_merit(
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

    pub fn amulet_increment(&mut self) -> Result<()> {
        self.total_amulet_count = self
            .total_amulet_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;
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
        self.total_donate_count = self
            .total_donate_count
            .checked_add(1)
            .ok_or(GlobalError::MathOverflow)?;
        self.check_and_temple_upgrade()?;
        Ok(())
    }

    pub fn check_and_temple_upgrade(&mut self) -> Result<()> {
        // 达到最高级后不再读取下一条件
        if (self.level as usize) >= UPGRADE_CONDITIONS.len() - 1 {
            return Ok(());
        }
        let (incense, lottery, wish, donate, amulet) = UPGRADE_CONDITIONS[self.level as usize];
        if self.total_incense_value >= incense
            && self.total_lottery_count >= lottery
            && self.total_wish_count >= wish
            && self.total_donate_amount >= donate
            && self.total_amulet_count >= amulet
        {
            self.level = self.level.checked_add(1).ok_or(GlobalError::MathOverflow)?;
        }
        Ok(())
    }

    pub fn increment_wealth(&mut self, amount: u64) -> Result<()> {
        self.wealth = self
            .wealth
            .checked_add(amount)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }

    pub fn increment_stake_merit(&mut self, merit: u64) -> Result<()> {
        self.total_merit_value = self
            .total_merit_value
            .checked_add(merit)
            .ok_or(GlobalError::MathOverflow)?;
        Ok(())
    }
}

// 寺庙升级条件 , 香火值,抽签,许愿次数,捐助sol
pub const UPGRADE_CONDITIONS: [(u64, u64, u64, u64, u64); 4] = [
    (0, 0, 0, 0, 0),
    (1, 1, 1, 1, 0),
    (2, 2, 2, 2, 1),
    (3, 3, 3, 3, 1),
];
