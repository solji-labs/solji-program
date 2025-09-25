use crate::error::ErrorCode;
use crate::state::shop_item::ShopItem;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(shop_items: Vec<ShopItem>, incense_points_rate: u64, merit_rate: u64)]
pub struct InitShopConfig<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        init,
        seeds = [
            ShopConfig::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref()
        ],
        bump,
        payer = authority,
        space = 8 + ShopConfig::INIT_SPACE
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_shop_config(
    ctx: Context<InitShopConfig>,
    shop_items: Vec<ShopItem>,
    incense_points_rate: u64,
    merit_rate: u64,
) -> Result<()> {
    let shop_config = &mut ctx.accounts.shop_config;
    let temple_config = &mut ctx.accounts.temple_config;

    // 初始化商城配置
    shop_config.temple_config = temple_config.key();
    shop_config.shop_items = shop_items;
    shop_config.incense_points_rate = incense_points_rate;
    shop_config.merit_rate = merit_rate;

    // 更新主配置账户的引用
    temple_config.shop_config = shop_config.key();

    msg!("Shop config initialized successfully");
    Ok(())
}

#[derive(Accounts)]
#[instruction(fortune_config: FortuneConfig, buddha_fortune_config: FortuneConfig)]
pub struct InitFortuneConfig<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        init,
        seeds = [
            FortuneConfigAccount::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref()
        ],
        bump,
        payer = authority,
        space = 8 + FortuneConfigAccount::INIT_SPACE
    )]
    pub fortune_config_account: Box<Account<'info, FortuneConfigAccount>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_fortune_config(
    ctx: Context<InitFortuneConfig>,
    fortune_config: FortuneConfig,
    buddha_fortune_config: FortuneConfig,
) -> Result<()> {
    let fortune_config_account = &mut ctx.accounts.fortune_config_account;
    let temple_config = &mut ctx.accounts.temple_config;

    // 验证概率总和为100
    let regular_total = fortune_config.great_luck_prob as u16
        + fortune_config.good_luck_prob as u16
        + fortune_config.neutral_prob as u16
        + fortune_config.bad_luck_prob as u16
        + fortune_config.great_bad_luck_prob as u16;

    let buddha_total = buddha_fortune_config.great_luck_prob as u16
        + buddha_fortune_config.good_luck_prob as u16
        + buddha_fortune_config.neutral_prob as u16
        + buddha_fortune_config.bad_luck_prob as u16
        + buddha_fortune_config.great_bad_luck_prob as u16;

    require!(regular_total == 100, ErrorCode::InvalidFortuneConfig);
    require!(buddha_total == 100, ErrorCode::InvalidFortuneConfig);

    // 初始化抽签配置
    fortune_config_account.temple_config = temple_config.key();
    fortune_config_account.fortune_config = fortune_config;
    fortune_config_account.buddha_fortune_config = buddha_fortune_config;

    // 更新主配置账户的引用
    temple_config.fortune_config = fortune_config_account.key();

    msg!("Fortune config initialized successfully");
    Ok(())
}

#[derive(Accounts)]
#[instruction(donation_levels: Vec<DonationLevelConfig>, donation_rewards: Vec<DonationRewardConfig>, temple_levels: Vec<TempleLevelConfig>)]
pub struct InitRewardConfig<'info> {
    #[account(
        mut,
        constraint = temple_config.owner == authority.key() @ ErrorCode::Unauthorized
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        init,
        seeds = [
            RewardConfig::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref()
        ],
        bump,
        payer = authority,
        space = 8 + RewardConfig::INIT_SPACE
    )]
    pub reward_config: Box<Account<'info, RewardConfig>>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn init_reward_config(
    ctx: Context<InitRewardConfig>,
    donation_levels: Vec<DonationLevelConfig>,
    donation_rewards: Vec<DonationRewardConfig>,
    temple_levels: Vec<TempleLevelConfig>,
) -> Result<()> {
    let reward_config = &mut ctx.accounts.reward_config;
    let temple_config = &mut ctx.accounts.temple_config;

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

    // 验证寺庙等级数据
    for level_config in &temple_levels {
        require!(
            level_config.level > 0 && level_config.level <= 5,
            ErrorCode::InvalidTempleLevel
        );
    }

    // 初始化奖励配置
    reward_config.temple_config = temple_config.key();
    reward_config.donation_levels = donation_levels;
    reward_config.donation_rewards = donation_rewards;
    reward_config.level_configs = temple_levels;

    // 更新主配置账户的引用
    temple_config.reward_config = reward_config.key();

    msg!("Reward config initialized successfully");
    Ok(())
}
