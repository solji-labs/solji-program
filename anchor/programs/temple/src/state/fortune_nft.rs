use anchor_lang::prelude::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub enum FortuneResult {
    GreatLuck,
    GoodLuck,
    Neutral,
    BadLuck,
    GreatBadLuck,
}

#[account]
#[derive(InitSpace)]
pub struct FortuneNFT {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub fortune_result: FortuneResult,
    pub minted_at: i64,
    pub merit_cost: u8,
    pub serial_number: u32,
}
