use anchor_lang::prelude::*;









#[error_code]
pub enum DonationError {
    #[msg("Invalid donation amount")]
    InvalidDonationAmount,

    #[msg("Insufficient payment")]
    InsufficientPayment,
}