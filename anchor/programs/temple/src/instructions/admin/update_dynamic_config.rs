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

// ===== 核心动态配置更新函数 =====

// 1. 更新烧香香型配置
pub fn update_incense_types(
    ctx: Context<UpdateDynamicConfig>,
    incense_types: Vec<IncenseType>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // 验证香型数据
    for incense in &incense_types {
        require!(incense.id > 0, ErrorCode::InvalidIncenseType);
        require!(!incense.name.is_empty(), ErrorCode::InvalidIncenseType);
        require!(incense.price_lamports > 0, ErrorCode::InvalidIncenseType);
    }

    // 更新香型配置
    temple_config.dynamic_config.incense_types = incense_types;

    msg!("Updated incense types configuration");
    Ok(())
}

// 2. 更新抽签签文配置
pub fn update_fortune_config(
    ctx: Context<UpdateDynamicConfig>,
    regular_fortune: FortuneConfig,
    buddha_fortune: FortuneConfig,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // 验证概率总和为100
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

    // 更新概率配置
    temple_config.dynamic_config.regular_fortune = regular_fortune;
    temple_config.dynamic_config.buddha_fortune = buddha_fortune;

    msg!("Updated fortune configuration");
    Ok(())
}

// 3. 更新捐助等级配置
pub fn update_donation_levels(
    ctx: Context<UpdateDynamicConfig>,
    donation_levels: Vec<DonationLevelConfig>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // 验证捐助等级数据
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

    // 更新捐助等级配置
    temple_config.dynamic_config.donation_levels = donation_levels;

    msg!("Updated donation levels configuration");
    Ok(())
}

// 3.5. 更新捐助奖励配置
pub fn update_donation_rewards(
    ctx: Context<UpdateDynamicConfig>,
    donation_rewards: Vec<DonationRewardConfig>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // 验证捐助奖励数据
    for reward_config in &donation_rewards {
        require!(
            reward_config.min_donation_sol >= 0.0,
            ErrorCode::InvalidDonationLevel
        );
        require!(reward_config.incense_id >= 0, ErrorCode::InvalidIncenseType);
    }

    // 更新捐助奖励配置
    temple_config.dynamic_config.donation_rewards = donation_rewards;

    msg!("Updated donation rewards configuration");
    Ok(())
}

// 4. 更新寺庙等级配置
pub fn update_temple_levels(
    ctx: Context<UpdateDynamicConfig>,
    temple_levels: Vec<TempleLevelConfig>,
) -> Result<()> {
    let temple_config = &mut ctx.accounts.temple_config;

    // 验证寺庙等级数据
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

// 5. 更新商城物品配置
pub fn update_shop_items(ctx: Context<UpdateShopItems>, shop_items: Vec<ShopItem>) -> Result<()> {
    let shop_config = &mut ctx.accounts.shop_config;
    let clock = Clock::get()?;

    // 验证商城物品数据
    for item in &shop_items {
        require!(item.id > 0, ErrorCode::InvalidShopItemId);
        require!(!item.name.is_empty(), ErrorCode::InvalidShopItemId);
        require!(item.price > 0, ErrorCode::InvalidShopItemId);
        require!(item.stock >= 0, ErrorCode::InvalidShopItemId);
    }

    // 更新商城物品配置
    shop_config.shop_items = shop_items;
    shop_config.update_timestamp(clock.unix_timestamp);

    msg!("Updated shop items configuration");
    Ok(())
}
