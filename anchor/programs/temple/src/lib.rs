#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*;

declare_id!("D9immZaczS2ASFqqSux2iCCAaFat7vcusB1PQ2SW6d95");

pub mod admin {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("DRayqG9RXYi8WHgWEmRQGrUWRWbhjYWYkCRJDd6JBBak");
    #[cfg(feature = "localnet")]
    pub const ID: Pubkey = pubkey!("FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU");
    #[cfg(not(any(feature = "devnet", feature = "localnet")))]
    pub const ID: Pubkey = pubkey!("FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU");
}

#[program]
pub mod temple {

    use super::*;

    
}
