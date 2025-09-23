use crate::error::ErrorCode;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(treasury: Pubkey, incense_types: Vec<IncenseType>, regular_fortune: FortuneConfig, buddha_fortune: FortuneConfig, donation_levels: Vec<DonationLevelConfig>)]
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

    pub system_program: Program<'info, System>,
}

pub fn create_temple_config(
    ctx: Context<CreateTempleConfig>,
    treasury: Pubkey,
    incense_types: Vec<IncenseType>,
    regular_fortune: FortuneConfig,
    buddha_fortune: FortuneConfig,
    donation_levels: Vec<DonationLevelConfig>,
) -> Result<()> {
    let temple_config: &mut Account<'_, TempleConfig> = &mut ctx.accounts.temple_config;
    let clock = Clock::get()?;

    // 初始化基础配置
    temple_config.owner = ctx.accounts.owner.key();
    temple_config.treasury = treasury;
    temple_config.total_incense_points = 0;
    temple_config.total_merit = 0;
    temple_config.level = 1;
    temple_config.created_at = clock.unix_timestamp;
    temple_config.total_buddha_nft = 0;

    // 初始化动态配置
    temple_config.dynamic_config = DynamicConfig {
        incense_types: incense_types.clone(),
        regular_fortune: regular_fortune.clone(),
        buddha_fortune: buddha_fortune.clone(),
        donation_levels: donation_levels.clone(),
    };

    msg!(
        "Temple config created successfully with {} incense types",
        incense_types.len()
    );
    Ok(())
}
