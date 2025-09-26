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
    FaintScent = 0,
    // 橙香
    OrangeIncense = 1,
    // 龙涎香
    Ambergris = 2,
    // 灵香
    Lingxiang = 3,
    // 秘制香
    SecretIncense = 4,
    // 天界香
    CelestialIncense = 5,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct IncenseBurnArgs {
    pub name: String,
    pub symbol: String,
    pub url: String,
    pub is_mutable: bool,         // 元数据是否可修改
    pub collection_details: bool, // 是否包含集合详情
    pub incense_type: IncenseType,
}
