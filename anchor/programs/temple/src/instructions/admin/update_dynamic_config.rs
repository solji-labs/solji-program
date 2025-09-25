use crate::error::ErrorCode;
use crate::state::shop_item::ShopItem;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;

// ===== 商城配置更新 =====
#[derive(Accounts)]
pub struct UpdateShopConfig<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        mut,
        address = temple_config.shop_config @ ErrorCode::InvalidAccount
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

// ===== 抽签配置更新 =====
#[derive(Accounts)]
pub struct UpdateFortuneConfig<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        mut,
        address = temple_config.fortune_config @ ErrorCode::InvalidAccount
    )]
    pub fortune_config: Box<Account<'info, FortuneConfigAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

// ===== 奖励配置更新 =====
#[derive(Accounts)]
pub struct UpdateRewardConfig<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        mut,
        address = temple_config.reward_config @ ErrorCode::InvalidAccount
    )]
    pub reward_config: Box<Account<'info, RewardConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

// ===== 商城配置更新函数 =====

// 更新商城物品配置
pub fn update_shop_items(ctx: Context<UpdateShopConfig>, shop_items: Vec<ShopItem>) -> Result<()> {
    let shop_config = &mut ctx.accounts.shop_config;

    // 验证商城物品数据
    for item in &shop_items {
        require!(item.id > 0, ErrorCode::InvalidShopItemId);
        require!(!item.name.is_empty(), ErrorCode::InvalidShopItemId);
        require!(item.price > 0, ErrorCode::InvalidShopItemId);
        require!(item.stock >= 0, ErrorCode::InvalidShopItemId);
    }

    // 更新商城物品配置
    shop_config.shop_items = shop_items;

    msg!("Updated shop items configuration");
    Ok(())
}

// ===== 抽签配置更新函数 =====

// 更新抽签签文配置
pub fn update_fortune_config(
    ctx: Context<UpdateFortuneConfig>,
    regular_fortune: FortuneConfig,
    buddha_fortune: FortuneConfig,
) -> Result<()> {
    let fortune_config = &mut ctx.accounts.fortune_config;

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
    fortune_config.fortune_config = regular_fortune;
    fortune_config.buddha_fortune_config = buddha_fortune;

    msg!("Updated fortune configuration");
    Ok(())
}

// ===== 奖励配置更新函数 =====

// 更新捐助等级配置
pub fn update_donation_levels(
    ctx: Context<UpdateRewardConfig>,
    donation_levels: Vec<DonationLevelConfig>,
) -> Result<()> {
    let reward_config = &mut ctx.accounts.reward_config;

    // 验证捐助等级数据
    for level_config in &donation_levels {
        require!(
            level_config.level > 0 && level_config.level <= 5,
            ErrorCode::InvalidDonationLevel
        );
        require!(
            level_config.min_amount_sol > 0.0,
            ErrorCode::InvalidDonationLevel
        );
    }

    // 更新捐助等级配置
    reward_config.donation_levels = donation_levels;

    msg!("Updated donation levels configuration");
    Ok(())
}

// 更新捐助奖励配置
pub fn update_donation_rewards(
    ctx: Context<UpdateRewardConfig>,
    donation_rewards: Vec<DonationRewardConfig>,
) -> Result<()> {
    let reward_config = &mut ctx.accounts.reward_config;

    // 验证捐助奖励数据
    for reward in &donation_rewards {
        require!(
            reward.min_donation_sol >= 0.0,
            ErrorCode::InvalidDonationLevel
        );
        require!(reward.incense_id >= 0, ErrorCode::InvalidIncenseType);
    }

    // 更新捐助奖励配置
    reward_config.donation_rewards = donation_rewards;

    msg!("Updated donation rewards configuration");
    Ok(())
}

// 更新寺庙等级配置
pub fn update_temple_levels(
    ctx: Context<UpdateRewardConfig>,
    temple_levels: Vec<TempleLevelConfig>,
) -> Result<()> {
    let reward_config = &mut ctx.accounts.reward_config;

    // 验证寺庙等级数据
    for level_config in &temple_levels {
        require!(
            level_config.level > 0 && level_config.level <= 5,
            ErrorCode::InvalidTempleLevel
        );
    }

    reward_config.level_configs = temple_levels;

    msg!("Updated temple levels configuration");
    Ok(())
}
