use anchor_lang::prelude::*;

pub struct Donation {}

impl Donation {
    pub fn calculate_donation(donate_amount_lamports: u64) -> Result<(u64, u64)> {
       let donate_amount_sol = donate_amount_lamports as f64 / 100_000_000.0;  // ✅ 先转换为浮点数再除法

        //增加功德值
        let reward_karma_points = if donate_amount_sol >= 5.0 {
            1200000
        } else if donate_amount_sol >= 1.0 {
            140000
        } else if donate_amount_sol >= 0.2 {
            1300
        } else if donate_amount_sol >= 0.05 {
            65
        } else {
            0
        };

        let reward_incense_value = if donate_amount_sol >= 5.0 {
            100000
        } else if donate_amount_sol >= 1.0 {
            30000
        } else if donate_amount_sol >= 0.2 {
            6300
        } else if donate_amount_sol >= 0.05 {
            1200
        } else {
            0
        };

        Ok((reward_karma_points, reward_incense_value))
    }
}

#[error_code]
pub enum DonationError {
    #[msg("Invalid donation amount")]
    InvalidDonationAmount,

    #[msg("Insufficient payment")]
    InsufficientPayment,
}
