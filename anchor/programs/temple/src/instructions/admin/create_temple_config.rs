use crate::error::ErrorCode;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(treasury: Pubkey, regular_fortune: FortuneConfig, buddha_fortune: FortuneConfig, donation_levels: Vec<DonationLevelConfig>, donation_rewards: Vec<DonationRewardConfig>, temple_levels: Vec<TempleLevelConfig>)]
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
        space = 8 + 7304
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
) -> Result<()> {
    let temple_config: &mut Account<'_, TempleConfig> = &mut ctx.accounts.temple_config;
    let clock = Clock::get()?;

    // Initialize basic configuration
    temple_config.owner = ctx.accounts.owner.key();
    temple_config.treasury = treasury;

    temple_config.level = 1;
    temple_config.created_at = clock.unix_timestamp;
    temple_config.total_buddha_nft = 0;
    temple_config.status = 0;
    temple_config.open_time = clock.unix_timestamp as u64; // onlin time

    // Initialize dynamic configuration
    temple_config.dynamic_config = DynamicConfig {
        incense_types: vec![
            IncenseType {
                id: 1,
                name: "Fresh".to_string(),
                price_lamports: 10000000, // 0.01 SOL
                merit: 10,
                incense_points: 100,
                is_donation: false,
            },
            IncenseType {
                id: 2,
                name: "Sandalwood".to_string(),
                price_lamports: 50000000, // 0.05 SOL
                merit: 65,
                incense_points: 600,
                is_donation: false,
            },
            IncenseType {
                id: 3,
                name: "Ambergris".to_string(),
                price_lamports: 100000000, // 0.1 SOL
                merit: 1200,
                incense_points: 3100,
                is_donation: false,
            },
            IncenseType {
                id: 4,
                name: "Supreme Spirit".to_string(),
                price_lamports: 300000000, // 0.3 SOL
                merit: 3400,
                incense_points: 9000,
                is_donation: false,
            },
            IncenseType {
                id: 5,
                name: "Secret".to_string(),
                price_lamports: 10000000000, // 10 SOL
                merit: 5000,
                incense_points: 15000,
                is_donation: true,
            },
            IncenseType {
                id: 6,
                name: "Heavenly".to_string(),
                price_lamports: 50000000000, // 50 SOL
                merit: 10000,
                incense_points: 30000,
                is_donation: true,
            },
        ],
        regular_fortune: regular_fortune.clone(),
        buddha_fortune: buddha_fortune.clone(),
        donation_levels: donation_levels.clone(),
        donation_rewards: donation_rewards.clone(),
        temple_levels: temple_levels.clone(),
    };

    // Global State
    let global_stats = &mut ctx.accounts.global_stats;
    global_stats.temple_config = temple_config.key();
    global_stats.total_incense_points = 0;
    global_stats.total_merit = 0;
    global_stats.total_draw_fortune = 0;
    global_stats.total_wishes = 0;
    global_stats.total_donations_lamports = 0;
    global_stats.total_users = 0;
    // NFT
    global_stats.total_fortune_nfts = 0;
    global_stats.total_amulets = 0;
    global_stats.total_buddha_lights = 0;
    global_stats.updated_at = clock.unix_timestamp;

    msg!("Temple config created successfully ");
    Ok(())
}
