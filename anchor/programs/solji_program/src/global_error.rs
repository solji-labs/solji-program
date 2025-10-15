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

    #[msg("No NFT Stake")]
    NoNFTToStake,

    #[msg("Not NFT Owner")]
    NotNFTOwner,

    #[msg("Cant Unstake Yet")]
    CantUnstakeYet,

    #[msg("Unstake Not Request")]
    UnstakeNotRequest,

    #[msg("Unstake Too Soon")]
    UnstakeTooSoon,
    #[msg("Unstake Requse Status Error")]
    UnstakeRequestStatusError,
    #[msg("Unstake Confirm Status Error")]
    UnstackConfirmStatusError,
}
