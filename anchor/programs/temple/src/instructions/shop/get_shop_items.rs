use crate::state::shop_config::ShopConfig;
use crate::state::shop_item::ShopItemInfo;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ShopItemsResult {
    pub items: Vec<ShopItemInfo>,
}

pub fn get_shop_items(ctx: Context<GetShopItems>) -> Result<ShopItemsResult> {
    let shop_config = &ctx.accounts.shop_config;

    let items_info: Vec<ShopItemInfo> = shop_config
        .shop_items
        .iter()
        .filter(|item| item.is_available)
        .map(|item| ShopItemInfo {
            id: item.id,
            name: item.name.clone(),
            description: item.description.clone(),
            price: item.price,
            item_type: item.item_type.clone(),
            stock: item.stock,
            is_available: item.is_available,
        })
        .collect();

    Ok(ShopItemsResult { items: items_info })
}

#[derive(Accounts)]
pub struct GetShopItems<'info> {
    #[account(
        seeds = [ShopConfig::SEED_PREFIX.as_bytes(), temple_config.key().as_ref()],
        bump,
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,
}
