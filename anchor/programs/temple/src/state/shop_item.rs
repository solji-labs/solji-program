use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Debug, InitSpace)]
pub enum ShopItemType {
    Incense = 0, // 香火
    Prop = 1,    // 道具
    Special = 2, // 特殊物品
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ShopItemInfo {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub price: u64, // lamports
    pub item_type: ShopItemType,
    pub stock: u64,
    pub is_available: bool,
}

#[account]
#[derive(InitSpace, Debug)]
pub struct ShopItem {
    pub id: u8,
    #[max_len(32)]
    pub name: String,
    #[max_len(256)]
    pub description: String,
    pub price: u64, // lamports
    pub item_type: ShopItemType,
    pub stock: u64,
    pub is_available: bool,
    // 香火物品的额外配置
    pub incense_config: Option<IncenseItemConfig>,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, Debug, InitSpace)]
pub struct IncenseItemConfig {
    pub merit: u64,          // 功德值
    pub incense_points: u64, // 香火值
}

impl ShopItem {
    pub const SEED_PREFIX: &'static str = "shop_item";

    pub fn new(
        id: u8,
        name: String,
        description: String,
        price: u64,
        item_type: ShopItemType,
        stock: u64,
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            item_type,
            stock,
            is_available: true,
            incense_config: None,
        }
    }

    pub fn new_incense(
        id: u8,
        name: String,
        description: String,
        price: u64,
        stock: u64,
        merit: u64,
        incense_points: u64,
    ) -> Self {
        Self {
            id,
            name,
            description,
            price,
            item_type: ShopItemType::Incense,
            stock,
            is_available: true,
            incense_config: Some(IncenseItemConfig {
                merit,
                incense_points,
            }),
        }
    }

    pub fn can_purchase(&self, quantity: u64) -> bool {
        self.is_available && self.stock >= quantity
    }

    pub fn reduce_stock(&mut self, quantity: u64) -> Result<()> {
        if !self.can_purchase(quantity) {
            return err!(crate::error::ErrorCode::InsufficientStock);
        }
        self.stock = self.stock.checked_sub(quantity).unwrap();
        Ok(())
    }

    pub fn get_total_price(&self, quantity: u64) -> Result<u64> {
        self.price
            .checked_mul(quantity)
            .ok_or(crate::error::ErrorCode::MathOverflow.into())
    }
}
