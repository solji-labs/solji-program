use anchor_lang::prelude::*;

pub struct BadgeNFT {}

impl BadgeNFT {
    pub const SEED_PREFIX: &'static str = "badge_nft_v1";
    pub const TOKEN_DECIMALS: u8 = 0;


    /// 计算捐助等级
    pub fn calculate_donation_level(total_donation_amount: u64) -> u8 {
        let donation_sol = (total_donation_amount as f64 / 1_000_000_000.0);

        if donation_sol >= 5.0 {
            4 // Supreme Patron
        } else if donation_sol >= 1.0 {
            3 // Gold Protector
        } else if donation_sol >= 0.2 {
            2 // Silver Disciple
        } else  {
            1 // Bronze Believer
        }  
    }

    pub fn get_nft_name(level: u8) -> String {
        match level {
            1 => "Bronze Believer".to_string(),
            2 => "Silver Disciple".to_string(),
            3 => "Gold Protector".to_string(),
            4 => "Supreme Patron".to_string(),
            _ => "Unknown level".to_string(),
        }
    }
}
