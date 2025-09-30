use crate::error::ErrorCode;
use crate::state::shop_config::ShopConfig;
use crate::state::shop_item::ShopItem;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct UpdateDynamicConfig<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct UpdateShopItems<'info> {
    #[account(
        mut,
        constraint = shop_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

// ===== Core dynamic configuration update functions =====

// 1. Update incense types configuration
pub fn update_incense_types(
    ctx: Context<UpdateDynamicConfig>,
    incense_types: Vec<IncenseType>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // Validate incense type data
    for incense in &incense_types {
        require!(incense.id > 0, ErrorCode::InvalidIncenseType);
        require!(!incense.name.is_empty(), ErrorCode::InvalidIncenseType);
        require!(incense.price_lamports > 0, ErrorCode::InvalidIncenseType);
    }

    // Update incense types configuration
    temple_config.dynamic_config.incense_types = incense_types;

    msg!("Updated incense types configuration");
    Ok(())
}

// 2. Update fortune configuration
pub fn update_fortune_config(
    ctx: Context<UpdateDynamicConfig>,
    regular_fortune: FortuneConfig,
    buddha_fortune: FortuneConfig,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // Validate probability total is 100
    let regular_total = regular_fortune.great_luck_prob as u16
        + regular_fortune.good_luck_prob as u16
        + regular_fortune.neutral_prob as u16
        + regular_fortune.bad_luck_prob as u16
        + regular_fortune.great_bad_luck_prob as u16;

    let buddha_total = buddha_fortune.great_luck_prob as u16
        + buddha_fortune.good_luck_prob as u16
        + buddha_fortune.neutral_prob as u16
        + buddha_fortune.bad_luck_prob as u16
        + buddha_fortune.great_bad_luck_prob as u16;

    require!(regular_total == 100, ErrorCode::InvalidFortuneConfig);
    require!(buddha_total == 100, ErrorCode::InvalidFortuneConfig);

    // Update probability configuration
    temple_config.dynamic_config.regular_fortune = regular_fortune;
    temple_config.dynamic_config.buddha_fortune = buddha_fortune;

    msg!("Updated fortune configuration");
    Ok(())
}

// 3. Update donation levels configuration
pub fn update_donation_levels(
    ctx: Context<UpdateDynamicConfig>,
    donation_levels: Vec<DonationLevelConfig>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // Validate donation level data
    for level_config in &donation_levels {
        require!(
            level_config.level > 0 && level_config.level <= 4,
            ErrorCode::InvalidDonationLevel
        );
        require!(
            level_config.min_amount_sol > 0.0,
            ErrorCode::InvalidDonationLevel
        );
    }

    // Update donation levels configuration
    temple_config.dynamic_config.donation_levels = donation_levels;

    msg!("Updated donation levels configuration");
    Ok(())
}

// 3.5. Update donation rewards configuration
pub fn update_donation_rewards(
    ctx: Context<UpdateDynamicConfig>,
    donation_rewards: Vec<DonationRewardConfig>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // Validate donation reward data
    for reward_config in &donation_rewards {
        require!(
            reward_config.min_donation_sol >= 0.0,
            ErrorCode::InvalidDonationLevel
        );
        require!(reward_config.incense_id >= 0, ErrorCode::InvalidIncenseType);
    }

    // Update donation rewards configuration
    temple_config.dynamic_config.donation_rewards = donation_rewards;

    msg!("Updated donation rewards configuration");
    Ok(())
}

// 4. Update temple levels configuration
pub fn update_temple_levels(
    ctx: Context<UpdateDynamicConfig>,
    temple_levels: Vec<TempleLevelConfig>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // Validate temple level data
    for level_config in &temple_levels {
        require!(
            level_config.level > 0 && level_config.level <= 4,
            ErrorCode::InvalidTempleLevel
        );
    }

    temple_config.dynamic_config.temple_levels = temple_levels;

    msg!("Updated success!");
    Ok(())
}

// 5. Update shop items configuration
pub fn update_shop_items(ctx: Context<UpdateShopItems>, shop_items: Vec<ShopItem>) -> Result<()> {
    let shop_config = &mut ctx.accounts.shop_config;
    let clock = Clock::get()?;

    // Validate shop item data
    for item in &shop_items {
        require!(item.id > 0, ErrorCode::InvalidShopItemId);
        require!(!item.name.is_empty(), ErrorCode::InvalidShopItemId);
        require!(item.price > 0, ErrorCode::InvalidShopItemId);
        require!(item.stock >= 0, ErrorCode::InvalidShopItemId);
    }

    // Update shop items configuration
    shop_config.shop_items = shop_items;
    shop_config.update_timestamp(clock.unix_timestamp);

    msg!("Updated shop items configuration");
    Ok(())
}
