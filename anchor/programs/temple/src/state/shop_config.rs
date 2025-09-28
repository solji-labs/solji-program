use crate::state::shop_item::{ShopItem, ShopItemType};
use anchor_lang::prelude::*;

// 商城配置账户 - 独立管理商城物品
#[account]
#[derive(InitSpace, Debug)]
pub struct ShopConfig {
    pub temple_config: Pubkey, // 关联的寺庙配置
    pub owner: Pubkey,         // 商城管理员（通常与寺庙管理员相同）
    pub created_at: i64,       // 创建时间
    pub updated_at: i64,       // 最后更新时间

    // 商城物品配置
    #[max_len(20)]
    pub shop_items: Vec<ShopItem>,
}

impl ShopConfig {
    pub const SEED_PREFIX: &str = "shop_config";

    // 获取商城物品
    pub fn find_item(&self, item_id: u8) -> Option<&ShopItem> {
        self.shop_items.iter().find(|item| item.id == item_id)
    }

    // 检查物品是否可用
    pub fn is_item_available(&self, item_id: u8) -> bool {
        self.find_item(item_id)
            .map(|item| item.is_available)
            .unwrap_or(false)
    }

    // 更新最后修改时间
    pub fn update_timestamp(&mut self, current_time: i64) {
        self.updated_at = current_time;
    }
}
