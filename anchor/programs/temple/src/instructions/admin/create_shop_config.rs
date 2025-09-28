use crate::error::ErrorCode;
use crate::state::shop_config::ShopConfig;
use crate::state::temple_config::TempleConfig;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(shop_items: Vec<crate::state::shop_item::ShopItem>)]
pub struct CreateShopConfig<'info> {
    #[account(
        mut,
        address = crate::admin::ID @ ErrorCode::InvalidOwner
    )]
    pub owner: Signer<'info>,

    #[account(
        init,
        seeds = [ShopConfig::SEED_PREFIX.as_bytes(), temple_config.key().as_ref()],
        bump,
        payer = owner,
        space = 8 + ShopConfig::INIT_SPACE
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_shop_config(
    ctx: Context<CreateShopConfig>,
    shop_items: Vec<crate::state::shop_item::ShopItem>,
) -> Result<()> {
    let shop_config = &mut ctx.accounts.shop_config;
    let temple_config = &ctx.accounts.temple_config;
    let clock = Clock::get()?;

    // 初始化商城配置
    shop_config.temple_config = temple_config.key();
    shop_config.owner = ctx.accounts.owner.key();
    shop_config.created_at = clock.unix_timestamp;
    shop_config.updated_at = clock.unix_timestamp;
    shop_config.shop_items = shop_items;

    msg!("Shop config created successfully");
    Ok(())
}
