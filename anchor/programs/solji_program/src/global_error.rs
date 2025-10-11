use anchor_lang::prelude::*;

#[error_code]
pub enum GlobalError {
    #[msg("Arithmetic overflow")]
    MathOverflow,

    #[msg("Arithmetic underflow")]
    MathUnderflow,

    #[msg("Invalid account")]
    InvalidAccount,

    #[msg("Invalid incense type")]
    InvalidIncenseType,

    #[msg("Not an administrator")]
    NonAdministrator,

    #[msg("Invalid level")]
    InvalidLevel,

    #[msg("Invalid args")]
    InvalidArgs,
    #[msg("Invalid Amulet")]
    InvalidAmulet,
}
