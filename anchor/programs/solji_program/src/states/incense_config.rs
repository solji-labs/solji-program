use anchor_lang::prelude::*;

// 定义香的结构体
#[account]
#[derive(InitSpace, Copy)]
pub struct IncenseRule {
    // 香的价格
    pub incense_price: u64,
    // 功德值
    pub merit_value: u64,
    // 香火值
    pub incense_value: u64,
}

impl IncenseRule {
    pub fn new(incense_price: u64, merit_value: u64, incense_value: u64) -> Self {
        Self {
            incense_price,
            merit_value,
            incense_value,
        }
    }
}

// 定义香的规则
#[account]
#[derive(InitSpace)]
pub struct IncenseRulesConfig {
    pub admin: Pubkey,
    // 香的规则
    #[max_len = 6]
    pub rules: [IncenseRule; 6],
}

impl IncenseRulesConfig {
    pub fn new(admin: Pubkey, rules: [IncenseRule; 6]) -> Self {
        Self { admin, rules }
    }

    pub fn update_rule(&mut self, incense_type: IncenseType, rule: IncenseRule) {
        self.rules[incense_type as usize] = rule;
    }

    pub fn get_rule(&self, incense_type: IncenseType) -> IncenseRule {
        self.rules[incense_type as usize]
    }
}
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum IncenseType {
    // 清香
    ClearIncense = 0,
    // 橙香
    Sandalwood = 1,
    // 龙涎香
    AmbergrisIncense = 2,
    // 太上灵香
    SupremeSpiritIncense = 3,
    // 秘制香
    SecretBrewIncense = 4,
    // 天界香
    CelestialIncense = 5,
}

impl IncenseType {
    pub fn get_incense_type(incense_type: u8) -> Option<IncenseType> {
        match incense_type {
            0 => Some(IncenseType::ClearIncense),
            1 => Some(IncenseType::Sandalwood),
            2 => Some(IncenseType::AmbergrisIncense),
            3 => Some(IncenseType::SupremeSpiritIncense),
            4 => Some(IncenseType::SecretBrewIncense),
            5 => Some(IncenseType::CelestialIncense),
            _ => None,
        }
    }

    pub fn get_nft_name(&self) -> String {
        match self {
            IncenseType::ClearIncense => "Merit Incense NFT: Pure".to_string(),
            IncenseType::Sandalwood => "Merit Incense NFT: Orange".to_string(),
            IncenseType::SupremeSpiritIncense => "Merit Incense NFT: Lingxiang".to_string(),
            IncenseType::AmbergrisIncense => "Merit Incense NFT: Ambergris".to_string(),
            IncenseType::SecretBrewIncense => "Merit Incense NFT: Secret Blend".to_string(),
            IncenseType::CelestialIncense => "Merit Incense NFT: Celestial".to_string(),
        }
    }

    pub fn get_symbol(&self) -> String {
        match self {
            IncenseType::ClearIncense => "PURE".to_string(),
            IncenseType::Sandalwood => "ORANGE".to_string(),
            IncenseType::SupremeSpiritIncense => "LINGXIANG".to_string(),
            IncenseType::AmbergrisIncense => "AMBERGRIS".to_string(),
            IncenseType::SecretBrewIncense => "SECRET".to_string(),
            IncenseType::CelestialIncense => "CELESTIAL".to_string(),
        }
    }

    pub fn get_nft_uri(&self) -> String {
        match self {
            IncenseType::ClearIncense => "https://example/faintscent.json".to_string(),
            IncenseType::Sandalwood => "https://example/orangeincense.json".to_string(),
            IncenseType::SupremeSpiritIncense => "https://example/lingxiang.json".to_string(),
            IncenseType::AmbergrisIncense => "https://example/ambergris.json".to_string(),
            IncenseType::SecretBrewIncense => "https://example/secretincense.json".to_string(),
            IncenseType::CelestialIncense => "https://example/celestialincense.json".to_string(),
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IncenseBurnArgs {
    pub incense_type: u8,
}
