use anchor_lang::prelude::*;












#[error_code]
pub enum BurnIncenseError {
    #[msg("Invalid payment amount")]
    InvalidPaymentAmount,
    #[msg("Not enough SOL")]
    NotEnoughSol,
    #[msg("Inactive incense type")]
    InactiveIncenseType,
    #[msg("Invalid owner")]
    InvalidOwner,
    #[msg("Daily burn limit exceeded")]
    DailyBurnLimitExceeded,
    #[msg("Invalid amount, must be between 1 and 10")]
    InvalidAmount,
    #[msg("Invalid incense type id")]
    InvalidIncenseTypeId,
    #[msg("Incense type not active")]
    IncenseTypeNotActive,
    #[msg("Not enough incense")]
    NotEnoughIncense,
}