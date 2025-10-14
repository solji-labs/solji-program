use crate::error::ErrorCode;
use crate::state::event::{DonationCompleted, DonationNFTMinted};
use crate::state::global_stats::GlobalStats;
use crate::state::medal_nft::*;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserDonationState, UserIncenseState, UserState};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

/// Complete donation instruction - handles fund transfer, rewards, and NFT minting in one transaction
#[derive(Accounts)]
pub struct DonateFund<'info> {
    #[account(mut)]
    pub donor: Signer<'info>,

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        mut,
        seeds = [GlobalStats::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub global_stats: Account<'info, GlobalStats>,

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        mut,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), donor.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    /// CHECK: Temple treasury account
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,

    // Medal NFT accounts (optional - only needed if minting medal NFT)
    #[account(
        init_if_needed,
        payer = donor,
        space = 8 + MedalNFT::INIT_SPACE,
        seeds = [MedalNFT::SEED_PREFIX.as_bytes(), b"account", temple_config.key().as_ref(), donor.key().as_ref()],
        bump,
    )]
    pub medal_nft_account: Box<Account<'info, MedalNFT>>,

    #[account(
        init_if_needed,
        payer = donor,
        seeds = [
            MedalNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            donor.key().as_ref()
        ],
        bump,
        mint::decimals = MedalNFT::TOKEN_DECIMALS,
        mint::authority = temple_config.key(),
        mint::freeze_authority = temple_config.key(),
    )]
    pub medal_nft_mint: Box<Account<'info, Mint>>,

    /// User's medal NFT associated account
    #[account(
        init_if_needed,
        payer = donor,
        associated_token::mint = medal_nft_mint,
        associated_token::authority = donor,
    )]
    pub medal_nft_token_account: Account<'info, TokenAccount>,

    /// CHECK: Medal NFT metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            medal_nft_mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub medal_nft_metadata: UncheckedAccount<'info>,

    // Program accounts
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn donate_fund(ctx: Context<DonateFund>, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::Donate,
        current_time,
    )?;

    let donor = &ctx.accounts.donor;
    let temple_treasury = &ctx.accounts.temple_treasury;

    // Validate donation amount
    require!(amount > 0, ErrorCode::InvalidAmount);

    // Transfer SOL to temple treasury
    let transfer_ix = anchor_lang::solana_program::system_instruction::transfer(
        &donor.key(),
        &temple_treasury.key(),
        amount,
    );

    anchor_lang::solana_program::program::invoke(
        &transfer_ix,
        &[
            donor.to_account_info(),
            temple_treasury.to_account_info(),
            ctx.accounts.system_program.to_account_info(),
        ],
    )?;

    // Process donation record
    ctx.accounts.user_donation_state.process_donation(amount);

    // Update global stats
    ctx.accounts.global_stats.add_donation(amount);

    // ===== PROCESS ALL DONATION REWARDS =====

    // 1. Get donation level rewards (merit points)
    let (merit_reward, incense_points_reward) =
        ctx.accounts.user_donation_state.get_donation_rewards();

    // 2. Apply merit and incense points rewards
    if merit_reward > 0 || incense_points_reward > 0 {
        ctx.accounts
            .user_incense_state
            .add_incense_value_and_merit(incense_points_reward, merit_reward);

        msg!(
            "Donation reward earned - Merit: {}, Incense points: {}",
            merit_reward,
            incense_points_reward
        );
    }

    // 3. Process incense unlock rewards (Secret Brew Incense, Celestial Incense)
    let total_donation_sol =
        ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0;

    // Iterate through all donation reward configurations
    for reward_config in &ctx.accounts.temple_config.dynamic_config.donation_rewards {
        // Check if minimum donation amount threshold is reached
        if total_donation_sol >= reward_config.min_donation_sol {
            // Calculate the total reward that should be obtained currently
            let current_reward = if reward_config.burn_bonus_per_001_sol > 0 {
                // Burn incense bonus: increase burn count per 0.01 SOL
                ((total_donation_sol * 100.0) as u64)
                    .saturating_mul(reward_config.burn_bonus_per_001_sol)
            } else {
                // Incense reward: cumulative reward based on threshold
                let current_tier = (total_donation_sol / reward_config.min_donation_sol) as u64;
                current_tier.saturating_mul(reward_config.incense_amount)
            };

            // Apply the reward
            if current_reward > 0 {
                if reward_config.burn_bonus_per_001_sol > 0 {
                    // Burn incense bonus
                    ctx.accounts.user_incense_state.incense_number = ctx
                        .accounts
                        .user_incense_state
                        .incense_number
                        .saturating_add(current_reward as u8);
                    msg!(
                        "Donation earned extra burn incense count: {}",
                        current_reward
                    );
                } else {
                    // Incense reward
                    ctx.accounts
                        .user_incense_state
                        .add_incense_balance(reward_config.incense_id, current_reward);
                    msg!(
                        "Donation unlocked incense type {}: {} sticks",
                        reward_config.incense_id,
                        current_reward
                    );
                }
            }
        }
    }

    // 4. Process special incense types (Secret Brew Incense, Celestial Incense)
    for special_incense in &ctx
        .accounts
        .temple_config
        .dynamic_config
        .special_incense_types
    {
        if total_donation_sol >= special_incense.required_donation_sol {
            // Calculate how many times this donation milestone has been reached
            let milestone_count =
                (total_donation_sol / special_incense.required_donation_sol) as u64;
            let total_reward = milestone_count.saturating_mul(special_incense.amount_per_donation);

            if total_reward > 0 {
                ctx.accounts
                    .user_incense_state
                    .add_incense_balance(special_incense.id, total_reward);

                msg!(
                    "Donation unlocked special incense {}: {} sticks",
                    special_incense.name,
                    total_reward
                );
            }
        }
    }

    // 5. Update global stats with rewards
    ctx.accounts
        .global_stats
        .add_incense_value_and_merit(incense_points_reward, merit_reward);

    // ===== MINT MEDAL NFT IF ELIGIBLE =====

    let donation_sol = amount as f64 / 1_000_000_000.0;

    // Check if user meets donation level requirements for medal NFT
    if donation_sol >= MedalNFT::get_level_min_donation_sol(1) {
        // Determine user's current level
        let mut current_level = 1;
        for level in (1..=4).rev() {
            if total_donation_sol >= MedalNFT::get_level_min_donation_sol(level) {
                current_level = level;
                break;
            }
        }

        // Check if user already has medal NFT
        if ctx.accounts.user_state.has_medal_nft {
            // User already has medal, check if can upgrade
            let next_upgrade_level = ctx
                .accounts
                .medal_nft_account
                .get_next_upgrade_level(total_donation_sol);
            if let Some(new_level) = next_upgrade_level {
                // Upgrade existing medal NFT
                let serial_number = ctx.accounts.medal_nft_account.serial_number;
                let new_name = if new_level == 4 {
                    format!("Supreme Dragon Medal #{}", serial_number)
                } else if new_level == 3 {
                    format!("Protector Gold Medal #{}", serial_number)
                } else if new_level == 2 {
                    format!("Diligent Silver Medal #{}", serial_number)
                } else {
                    format!("Entry Merit Bronze Medal #{}", serial_number)
                };

                let new_uri = format!(
                    "https://api.foxverse.co/temple/medal/{}/metadata.json",
                    new_level
                );

                // Update metadata (simplified - in real implementation would use update_metadata_accounts_v2)
                // For now, just update the account data
                let now = Clock::get()?.unix_timestamp;
                ctx.accounts.medal_nft_account.level = new_level;
                ctx.accounts.medal_nft_account.total_donation =
                    ctx.accounts.user_donation_state.donation_amount;
                ctx.accounts.medal_nft_account.last_upgrade = now;

                msg!("Temple medal NFT upgrade successful: {}", new_name);
                msg!("New level: {}", new_level);

                // Emit NFT upgrade event
                emit!(DonationNFTMinted {
                    user: donor.key(),
                    nft_mint: ctx.accounts.medal_nft_mint.key(),
                    level: new_level,
                    serial_number,
                    timestamp: clock.unix_timestamp,
                });
            }
        } else {
            // Mint new medal NFT
            let serial_number = ctx.accounts.user_donation_state.total_donation_count;

            let medal_name = if current_level == 4 {
                format!("Supreme Dragon Medal #{}", serial_number)
            } else if current_level == 3 {
                format!("Protector Gold Medal #{}", serial_number)
            } else if current_level == 2 {
                format!("Diligent Silver Medal #{}", serial_number)
            } else {
                format!("Entry Merit Bronze Medal #{}", serial_number)
            };

            // Create metadata account
            let temple_signer_seeds: &[&[&[u8]]] = &[&[
                TempleConfig::SEED_PREFIX.as_bytes(),
                &[ctx.bumps.temple_config],
            ]];

            create_metadata_accounts_v3(
                CpiContext::new_with_signer(
                    ctx.accounts.token_metadata_program.to_account_info(),
                    CreateMetadataAccountsV3 {
                        metadata: ctx.accounts.medal_nft_metadata.to_account_info(),
                        mint: ctx.accounts.medal_nft_mint.to_account_info(),
                        mint_authority: ctx.accounts.temple_config.to_account_info(),
                        update_authority: ctx.accounts.donor.to_account_info(),
                        payer: ctx.accounts.donor.to_account_info(),
                        system_program: ctx.accounts.system_program.to_account_info(),
                        rent: ctx.accounts.rent.to_account_info(),
                    },
                    temple_signer_seeds,
                ),
                DataV2 {
                    name: medal_name.clone(),
                    symbol: "TMM".to_string(),
                    uri: format!(
                        "https://api.foxverse.co/temple/medal/{}/metadata.json",
                        current_level
                    ),
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                },
                true, // Allow metadata to be mutable for future upgrades
                true,
                None,
            )?;

            // Mint medal NFT
            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.medal_nft_mint.to_account_info(),
                        to: ctx.accounts.medal_nft_token_account.to_account_info(),
                        authority: ctx.accounts.temple_config.to_account_info(),
                    },
                    temple_signer_seeds,
                ),
                1,
            )?;

            // Initialize MedalNFT account data
            let now = Clock::get()?.unix_timestamp;
            ctx.accounts.medal_nft_account.owner = ctx.accounts.donor.key();
            ctx.accounts.medal_nft_account.mint = ctx.accounts.medal_nft_mint.key();
            ctx.accounts.medal_nft_account.level = current_level;
            ctx.accounts.medal_nft_account.total_donation =
                ctx.accounts.user_donation_state.donation_amount;
            ctx.accounts.medal_nft_account.minted_at = now;
            ctx.accounts.medal_nft_account.last_upgrade = now;
            ctx.accounts.medal_nft_account.merit = ctx.accounts.user_incense_state.merit;
            ctx.accounts.medal_nft_account.serial_number = serial_number;

            // Update user state
            ctx.accounts.user_state.has_medal_nft = true;

            // Update global stats
            ctx.accounts.global_stats.increment_fortune_nfts();

            msg!("Temple medal NFT mint successful: {}", medal_name);
            msg!("Medal level: {}", current_level);

            // Emit NFT mint event
            emit!(DonationNFTMinted {
                user: ctx.accounts.donor.key(),
                nft_mint: ctx.accounts.medal_nft_mint.key(),
                level: current_level,
                serial_number,
                timestamp: clock.unix_timestamp,
            });
        }
    }

    // ===== EMIT DONATION COMPLETED EVENT =====

    emit!(DonationCompleted {
        user: donor.key(),
        amount,
        total_donated: ctx.accounts.user_donation_state.donation_amount,
        level: ctx.accounts.user_donation_state.donation_level,
        timestamp: clock.unix_timestamp,
    });

    let donation_sol = amount as f64 / 1_000_000_000.0;
    msg!("User {} donated {:.6} SOL", donor.key(), donation_sol);
    msg!(
        "Current donation level: {}",
        ctx.accounts.user_donation_state.donation_level
    );
    msg!("All donation rewards processed successfully!");

    Ok(())
}
