use crate::error::ErrorCode;
use crate::state::event::DonationNFTMinted;
use crate::state::global_stats::GlobalStats;
use crate::state::medal_nft::*;
use crate::state::temple_config::*;
use crate::state::user_state::{UserDonationState, UserIncenseState, UserState};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::update_metadata_accounts_v2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::metadata::UpdateMetadataAccountsV2;
use anchor_spl::token::mint_to;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
/// Medal
/// Called after donation, will update or mint new medal NFT
pub fn mint_medal_nft(ctx: Context<MintMedalNFT>) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::MintNFT,
        current_time,
    )?;

    let user_state = &mut ctx.accounts.user_state;
    let donation_sol = ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0;

    // Check if user already has medal NFT
    if user_state.has_medal_nft {
        // User already has medal, check if can upgrade
        let next_upgrade_level = ctx
            .accounts
            .medal_nft_account
            .get_next_upgrade_level(donation_sol);
        if let Some(new_level) = next_upgrade_level {
            // Build updated metadata
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

            // Use update_metadata_accounts_v2 to update metadata
            update_metadata_accounts_v2(
                CpiContext::new(
                    ctx.accounts.token_metadata_program.to_account_info(),
                    UpdateMetadataAccountsV2 {
                        metadata: ctx.accounts.meta_account.to_account_info(),
                        update_authority: ctx.accounts.authority.to_account_info(),
                    },
                ),
                None, // Keep existing update_authority
                Some(DataV2 {
                    name: new_name.clone(),
                    symbol: "TMM".to_string(),
                    uri: new_uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                }),
                None,
                if new_level == 4 { Some(true) } else { None }, // Only set to immutable for highest level
            )?;

            // Update MedalNFT account data
            let now = Clock::get()?.unix_timestamp;
            ctx.accounts.medal_nft_account.level = new_level;
            ctx.accounts.medal_nft_account.total_donation =
                ctx.accounts.user_donation_state.donation_amount;
            ctx.accounts.medal_nft_account.last_upgrade = now;

            msg!("Temple medal NFT upgrade successful: {}", new_name);
            msg!("New level: {}", new_level);
            msg!("Total donation amount: {:.6} SOL", donation_sol);

            // Emit NFT mint event
            emit!(DonationNFTMinted {
                user: ctx.accounts.authority.key(),
                nft_mint: ctx.accounts.nft_mint_account.key(),
                level: new_level,
                serial_number,
                timestamp: clock.unix_timestamp,
            });
        } else {
            msg!("Current donation amount insufficient for medal NFT upgrade");
            return err!(ErrorCode::InsufficientDonationForUpgrade);
        }
    } else {
        // Mint new NFT
        // Check if user meets donation level requirements
        if donation_sol < MedalNFT::get_level_min_donation_sol(1) {
            return err!(ErrorCode::InsufficientDonationForMedal);
        }

        // Determine user's current level
        let mut current_level = 1;
        for level in (1..=4).rev() {
            if donation_sol >= MedalNFT::get_level_min_donation_sol(level) {
                current_level = level;
                break;
            }
        }

        // Generate serial number
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
                    metadata: ctx.accounts.meta_account.to_account_info(),
                    mint: ctx.accounts.nft_mint_account.to_account_info(),
                    mint_authority: ctx.accounts.temple_config.to_account_info(),
                    update_authority: ctx.accounts.authority.to_account_info(),
                    payer: ctx.accounts.authority.to_account_info(),
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
                    mint: ctx.accounts.nft_mint_account.to_account_info(),
                    to: ctx.accounts.nft_associated_token_account.to_account_info(),
                    authority: ctx.accounts.temple_config.to_account_info(),
                },
                temple_signer_seeds,
            ),
            1,
        )?;

        // Initialize MedalNFT account data
        let now = Clock::get()?.unix_timestamp;
        ctx.accounts.medal_nft_account.owner = ctx.accounts.authority.key();
        ctx.accounts.medal_nft_account.mint = ctx.accounts.nft_mint_account.key();
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
        msg!("Total donation amount: {:.6} SOL", donation_sol);

        // Emit NFT mint event
        emit!(DonationNFTMinted {
            user: ctx.accounts.authority.key(),
            nft_mint: ctx.accounts.nft_mint_account.key(),
            level: current_level,
            serial_number,
            timestamp: clock.unix_timestamp,
        });
    }

    Ok(())
}

#[derive(Accounts)]
pub struct MintMedalNFT<'info> {
    /// User account
    #[account(mut)]
    pub authority: Signer<'info>,

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

    /// User state account
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    /// User donation state account
    #[account(
        mut,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    /// User incense state account
    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + MedalNFT::INIT_SPACE,
        seeds = [MedalNFT::SEED_PREFIX.as_bytes(), b"account", temple_config.key().as_ref(), authority.key().as_ref()],
        bump,
    )]
    pub medal_nft_account: Box<Account<'info, MedalNFT>>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [
            MedalNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            authority.key().as_ref()
        ],
        bump,
        mint::decimals = MedalNFT::TOKEN_DECIMALS,
        mint::authority = temple_config.key(),
        mint::freeze_authority = temple_config.key(),
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// User's medal NFT associated account
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: this is the metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub meta_account: UncheckedAccount<'info>,

    // Program accounts
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
