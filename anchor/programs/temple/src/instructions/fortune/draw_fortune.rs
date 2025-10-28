use crate::error::ErrorCode;
use crate::state::event::FortuneDrawn;
use crate::state::fortune_nft::{FortuneNFT, FortuneResult};
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
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

    pub fn get_uri(&self) -> &str {
        match self {
            FortuneResult::GreatLuck => {
                "https://solji.mypinata.cloud/ipfs/QmeYUSLgMKYL8128quaieDUXfbdeKVsGBRQLmWjgAsqw2y"
            }
            FortuneResult::GoodLuck => {
                "https://solji.mypinata.cloud/ipfs/Qmdkcptk4783ej2sKNsK39UrNXXjjCxoFLbsYLt5KbhzDA"
            }
            FortuneResult::Neutral => {
                "https://solji.mypinata.cloud/ipfs/QmZkYr6vMhSYA37TEPpN2pAC7Cw3DZSHQKo6mNfadbwKik"
            }
            FortuneResult::BadLuck => {
                "https://solji.mypinata.cloud/ipfs/QmSiaGHzMyCijCSRf5tc9oFh7Ajs7Nnr2DMq6WVtiM4D8B"
            }
            FortuneResult::GreatBadLuck => {
                "https://solji.mypinata.cloud/ipfs/QmVQSbuJJnwYQ9ZJYvMcTDyxc1X75mjdVxNkjxWf6D2HsT"
            }
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

    // Fortune NFT accounts
    #[account(
        init_if_needed,
        payer = user,
        seeds = [
            b"fortune_nft",
            temple_config.key().as_ref(),
            user.key().as_ref(),
            &user_incense_state.total_draws.to_string().as_bytes(),
        ],
        bump,
        space = 8 + FortuneNFT::INIT_SPACE,
    )]
    pub fortune_nft_account: Box<Account<'info, FortuneNFT>>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [
            b"fortune_nft_mint",
            temple_config.key().as_ref(),
            user.key().as_ref(),
            &user_incense_state.total_draws.to_string().as_bytes(),
        ],
        bump,
        mint::decimals = 0,
        mint::authority = temple_config.key(),
        mint::freeze_authority = temple_config.key(),
    )]
    pub fortune_nft_mint: Box<Account<'info, Mint>>,

    /// User's fortune NFT associated account
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = fortune_nft_mint,
        associated_token::authority = user,
    )]
    pub fortune_nft_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK: Fortune NFT metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            fortune_nft_mint.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub fortune_nft_metadata: UncheckedAccount<'info>,

    /// CHECK: Randomness account (only needed in mainnet)
    #[cfg(feature = "mainnet")]
    pub randomness_account: Option<AccountInfo<'info>>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
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
    #[cfg(feature = "mainnet")]
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

    #[cfg(not(feature = "mainnet"))]
    let random_value = {
        // Test environments (localnet, devnet): use pseudo-random seed
        let clock = Clock::get()?;
        let seed = clock.unix_timestamp as u64 + clock.slot;
        (seed % 100) as u8
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

    // Mint fortune NFT
    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    let fortune_str = fortune.as_str();

    // Create metadata account
    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.fortune_nft_metadata.to_account_info(),
                mint: ctx.accounts.fortune_nft_mint.to_account_info(),
                mint_authority: ctx.accounts.temple_config.to_account_info(),
                update_authority: ctx.accounts.user.to_account_info(),
                payer: ctx.accounts.user.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            temple_signer_seeds,
        ),
        DataV2 {
            name: format!("Fortune NFT - {}", fortune_str),
            symbol: "TMF".to_string(),
            uri: fortune.get_uri().to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true, // Allow metadata to be mutable for future upgrades
        true,
        None,
    )?;

    // Mint fortune NFT
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.fortune_nft_mint.to_account_info(),
                to: ctx.accounts.fortune_nft_token_account.to_account_info(),
                authority: ctx.accounts.temple_config.to_account_info(),
            },
            temple_signer_seeds,
        ),
        1,
    )?;

    // Initialize FortuneNFT account data
    ctx.accounts.fortune_nft_account.owner = ctx.accounts.user.key();
    ctx.accounts.fortune_nft_account.mint = ctx.accounts.fortune_nft_mint.key();
    ctx.accounts.fortune_nft_account.fortune_result = fortune.clone();
    ctx.accounts.fortune_nft_account.minted_at = now;
    ctx.accounts.fortune_nft_account.merit_cost = if use_merit { 5 } else { 0 };
    ctx.accounts.fortune_nft_account.serial_number = ctx.accounts.user_incense_state.total_draws;

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
    msg!("Fortune NFT minted successfully!");

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

    // Emit fortune NFT minted event
    emit!(crate::state::event::FortuneNFTMinted {
        user: ctx.accounts.user.key(),
        fortune_nft_mint: ctx.accounts.fortune_nft_mint.key(),
        fortune_result: fortune.as_str().to_string(),
        merit_cost: if use_merit { 5 } else { 0 },
        serial_number: ctx.accounts.user_incense_state.total_draws,
        timestamp: now,
    });

    let result = DrawResult {
        fortune,
        timestamp: now,
        used_merit: use_merit,
    };

    Ok(result)
}
