use crate::error::ErrorCode;
use crate::state::global_stats::GlobalStats;
use crate::state::shop_item::ShopItem;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(treasury: Pubkey, regular_fortune: FortuneConfig, buddha_fortune: FortuneConfig, donation_levels: Vec<DonationLevelConfig>, donation_rewards: Vec<DonationRewardConfig>, temple_levels: Vec<TempleLevelConfig>, shop_items: Vec<ShopItem>)]
pub struct CreateTempleConfig<'info> {
    #[account(
        mut,
        address = crate::admin::ID @ ErrorCode::InvalidOwner
    )]
    pub owner: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
        payer = owner,
        space = 8 + TempleConfig::INIT_SPACE
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        init,
        seeds = [GlobalStats::SEED_PREFIX.as_bytes()],
        bump,
        payer = owner,
        space = 8 + GlobalStats::INIT_SPACE
    )]
    pub global_stats: Account<'info, GlobalStats>,

    // 配置账户
    #[account(
        init,
        seeds = [
            ShopConfig::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + ShopConfig::INIT_SPACE
    )]
    pub shop_config: Box<Account<'info, ShopConfig>>,

    #[account(
        init,
        seeds = [
            FortuneConfigAccount::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + FortuneConfigAccount::INIT_SPACE
    )]
    pub fortune_config: Box<Account<'info, FortuneConfigAccount>>,

    #[account(
        init,
        seeds = [
            RewardConfig::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref()
        ],
        bump,
        payer = owner,
        space = 8 + RewardConfig::INIT_SPACE
    )]
    pub reward_config: Box<Account<'info, RewardConfig>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_temple_config(
    ctx: Context<CreateTempleConfig>,
    treasury: Pubkey,
    regular_fortune: FortuneConfig,
    buddha_fortune: FortuneConfig,
    donation_levels: Vec<DonationLevelConfig>,
    donation_rewards: Vec<DonationRewardConfig>,
    temple_levels: Vec<TempleLevelConfig>,
    shop_items: Vec<ShopItem>,
) -> Result<()> {
    let temple_config: &mut Account<'_, TempleConfig> = &mut ctx.accounts.temple_config;
    let clock = Clock::get()?;

    // 初始化基础配置
    temple_config.owner = ctx.accounts.owner.key();
    temple_config.treasury = treasury;

    temple_config.level = 1;
    temple_config.created_at = clock.unix_timestamp;
    temple_config.total_buddha_nft = 0;
    temple_config.status = 0; // 初始状态：全部启用
    temple_config.open_time = clock.unix_timestamp as u64; // 立即上线

    // 初始化配置账户引用
    temple_config.shop_config = ctx.accounts.shop_config.key();
    temple_config.fortune_config = ctx.accounts.fortune_config.key();
    temple_config.reward_config = ctx.accounts.reward_config.key();

    // 初始化商城配置
    let shop_config = &mut ctx.accounts.shop_config;
    shop_config.temple_config = temple_config.key();
    shop_config.shop_items = shop_items;
    shop_config.incense_points_rate = 100; // 默认兑换比例
    shop_config.merit_rate = 100; // 默认兑换比例

    // 初始化抽签配置
    let fortune_config_account = &mut ctx.accounts.fortune_config;
    fortune_config_account.temple_config = temple_config.key();
    fortune_config_account.fortune_config = regular_fortune;
    fortune_config_account.buddha_fortune_config = buddha_fortune;

    // 初始化奖励配置
    let reward_config = &mut ctx.accounts.reward_config;
    reward_config.temple_config = temple_config.key();
    reward_config.donation_levels = donation_levels;
    reward_config.donation_rewards = donation_rewards;
    reward_config.level_configs = temple_levels;

    // 初始化全局统计
    let global_stats = &mut ctx.accounts.global_stats;
    global_stats.temple_config = temple_config.key();
    // 核心统计数据
    global_stats.total_incense_points = 0;
    global_stats.total_merit = 0;
    global_stats.total_draw_fortune = 0;
    global_stats.total_wishes = 0;
    global_stats.total_donations_lamports = 0;
    global_stats.total_users = 0;
    // NFT统计
    global_stats.total_fortune_nfts = 0;
    global_stats.total_amulets = 0;
    global_stats.total_buddha_lights = 0;
    // 元数据
    global_stats.updated_at = clock.unix_timestamp;

    msg!("Temple config created successfully with all config accounts");
    Ok(())
}
