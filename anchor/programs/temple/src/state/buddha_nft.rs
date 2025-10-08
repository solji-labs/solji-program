use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct BuddhaNft {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub serial_number: u32,
    pub minted_at: i64,
    pub is_active: bool,
}


impl BuddhaNft {
    pub const SEED_PREFIX: &'static str = "buddha_nft";
    pub const TOKEN_DECIMALS: u8 = 0;
    pub const TOKEN_NAME: &'static str = "BuddhaNFT";
    pub const TOKEN_SYMBOL: &'static str = "MTK";
    pub const TOKEN_URI: &'static str = "https://temple.mintkit.io/token.json";


    pub fn initialize(&mut self, owner: Pubkey, mint: Pubkey, serial_number: u32, minted_at: i64) {
        self.owner = owner;
        self.mint = mint;
        self.serial_number = serial_number;
        self.minted_at = minted_at;
        self.is_active = true;
    }
}

#[error_code]
pub enum BuddhaNftError {
    #[msg("User already minted Buddha NFT")]
    UserAlreadyMintedBuddhaNft,
    #[msg("User cannot mint Buddha NFT")]
    UserCannotMintBuddhaNft,
}
    