use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug)]
pub struct IncenseNFT {}
impl IncenseNFT {
    pub const SEED_PREFIX: &'static str = "IncenseNFT_V1";
    pub const TOKEN_DECIMALS: u8 = 0;
    pub const TOKEN_NAME: &'static str = "IncenseNFT_V1";
    pub const TOKEN_SYMBOL: &'static str = "SOLJI";
}
