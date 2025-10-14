use crate::error::ErrorCode;
use crate::state::event::FortuneDrawn;
// use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;

// Define fortune result enum
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum FortuneResult {
    GreatLuck,    // Great luck
    GoodLuck,     // Good luck
    Neutral,      // Neutral
    BadLuck,      // Bad luck
    GreatBadLuck, // Great bad luck
}

impl FortuneResult {
    pub fn as_str(&self) -> &str {
        match self {
            FortuneResult::GreatLuck => "Great Luck",
            FortuneResult::GoodLuck => "Good Luck",
            FortuneResult::Neutral => "Neutral",
            FortuneResult::BadLuck => "Bad Luck",
            FortuneResult::GreatBadLuck => "Great Bad Luck",
        }
    }

    pub fn get_description(&self) -> &str {
        match self {
            FortuneResult::GreatLuck => "Everything goes smoothly, wishes come true",
            FortuneResult::GoodLuck => "All things go well, gradually improving",
            FortuneResult::Neutral => "Plain and simple, steady progress",
            FortuneResult::BadLuck => "Be careful, turn misfortune into fortune",
            FortuneResult::GreatBadLuck => "Be extra careful, observe quietly",
        }
    }
}

// Define draw result structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DrawResult {
    pub fortune: FortuneResult,
    pub timestamp: i64,
    pub used_merit: bool,
}

#[derive(Accounts)]
pub struct DrawFortune<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump = user_state.bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    /// CHECK: Randomness account (only needed in non-local environment)
    #[cfg(not(feature = "localnet"))]
    pub randomness_account: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

pub fn draw_fortune(
    ctx: Context<DrawFortune>,
    use_merit: bool,
    has_fortune_amulet: bool,
    has_protection_amulet: bool,
) -> Result<DrawResult> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;
    let now = clock.unix_timestamp;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::DrawFortune,
        current_time,
    )?;

    let user_state: &mut Account<'_, UserState> = &mut ctx.accounts.user_state;

    // Check if merit can be used for drawing fortune
    if use_merit {
        ctx.accounts.user_incense_state.consume_merit_for_draw(5)?;
    } else {
        // Check if free draw is available
        if !ctx.accounts.user_incense_state.can_draw_free() {
            return err!(ErrorCode::DailyIncenseLimitExceeded);
        }
    }

    // Generate random number: decide method based on compilation features
    #[cfg(feature = "localnet")]
    let random_value = {
        // Local test environment: use system clock as pseudo-random seed
        let clock = Clock::get()?;
        let seed = clock.unix_timestamp as u64 + clock.slot;
        (seed % 100) as u8
    };

    #[cfg(not(feature = "localnet"))]
    let random_value = {
        // Production environment: use Switchboard oracle randomness
        let clock = Clock::get()?;

        // Parse randomness account data
        let randomness_data = switchboard_on_demand::RandomnessAccountData::parse(
            ctx.accounts.randomness_account.data.borrow(),
        )
        .map_err(|_| ErrorCode::InvalidRandomness)?;

        // Get random value
        let revealed_random_value = randomness_data
            .get_value(clock.slot)
            .map_err(|_| ErrorCode::RandomnessNotResolved)?;

        // Extract a u64 value from random number array
        let mut random_bytes = [0u8; 8];
        random_bytes.copy_from_slice(&revealed_random_value[..8]);
        let random_u64 = u64::from_le_bytes(random_bytes);

        // Convert to 0-99 random number
        (random_u64 % 100) as u8
    };

    // Get probability settings from dynamic config
    let mut fortune_config = ctx
        .accounts
        .temple_config
        .get_fortune_config(user_state.has_buddha_nft)
        .clone();

    // Apply amulet effects to fortune probabilities
    if has_fortune_amulet {
        // Good Luck Amulet: +20% to Great Luck and Good Luck probabilities
        let bonus = 20;
        fortune_config.great_luck_prob = fortune_config.great_luck_prob.saturating_add(bonus);
        fortune_config.good_luck_prob = fortune_config.good_luck_prob.saturating_add(bonus);
        msg!("Fortune Amulet activated: +20% to Great Luck and Good Luck probabilities");
    }

    if has_protection_amulet {
        // Protection Amulet: -20% to Bad Luck and Great Bad Luck probabilities
        let reduction = 20;
        fortune_config.bad_luck_prob = fortune_config.bad_luck_prob.saturating_sub(reduction);
        fortune_config.great_bad_luck_prob =
            fortune_config.great_bad_luck_prob.saturating_sub(reduction);
        msg!("Protection Amulet activated: -20% to Bad Luck and Great Bad Luck probabilities");
    }

    if user_state.has_buddha_nft {
        msg!("Buddha NFT holder gets probability bonus");
    }

    let fortune = {
        let mut cumulative_prob = 0u8;
        cumulative_prob += fortune_config.great_luck_prob;
        if random_value < cumulative_prob {
            FortuneResult::GreatLuck
        } else {
            cumulative_prob += fortune_config.good_luck_prob;
            if random_value < cumulative_prob {
                FortuneResult::GoodLuck
            } else {
                cumulative_prob += fortune_config.neutral_prob;
                if random_value < cumulative_prob {
                    FortuneResult::Neutral
                } else {
                    cumulative_prob += fortune_config.bad_luck_prob;
                    if random_value < cumulative_prob {
                        FortuneResult::BadLuck
                    } else {
                        FortuneResult::GreatBadLuck
                    }
                }
            }
        }
    };

    // Update user draw count
    ctx.accounts.user_incense_state.update_draw_count();

    // // Update global stats
    // ctx.accounts.global_stats.increment_draw_fortune();

    // // Update temple level
    // ctx.accounts
    //     .temple_config
    //     .update_level(&ctx.accounts.global_stats);

    // Give merit reward
    if !use_merit {
        ctx.accounts
            .user_incense_state
            .add_incense_value_and_merit(0, 2);
    }

    let fortune_str = fortune.as_str();
    let fortune_desc = fortune.get_description();

    msg!("Draw result: {}", fortune_str);
    msg!("Fortune explanation: {}", fortune_desc);

    // TODO Amulet drop probability logic: 10% chance
    #[cfg(feature = "localnet")]
    let amulet_dropped = true; // Test environment: 100% drop rate for amulet

    #[cfg(not(feature = "localnet"))]
    let amulet_dropped = {
        let amulet_drop_random = (random_value.wrapping_add(42) % 100) as u8;
        amulet_drop_random < 10
    };
    if amulet_dropped {
        // Emit amulet dropped event with type information
        msg!("Congratulations! Got 1 Fortune Amulet NFT from drawing fortune!");
        emit!(crate::state::event::AmuletDropped {
            user: ctx.accounts.user.key(),
            amulet_type: 0, // Fortune Amulet
            source: "draw_fortune".to_string(),
            timestamp: now,
        });
    }

    // Emit draw fortune event
    emit!(FortuneDrawn {
        user: ctx.accounts.user.key(),
        fortune_result: fortune.as_str().to_string(),
        used_merit: use_merit,
        amulet_dropped,
        timestamp: now,
    });

    let result = DrawResult {
        fortune,
        timestamp: now,
        used_merit: use_merit,
    };

    Ok(result)
}
