use crate::error::ErrorCode;
use crate::incense_nft::IncenseNFT;
use crate::state::event::IncenseBurned;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::*;
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::burn;
use anchor_spl::token::mint_to;
use anchor_spl::token::Burn;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
pub fn burn_incense(ctx: Context<BurnIncense>, incense_id: u8, amount: u64) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::BurnIncense,
        current_time,
    )?;

    let incense_type = ctx
        .accounts
        .temple_config
        .find_incense_type(incense_id)
        .ok_or(ErrorCode::InvalidIncenseId)?;

    let incense_points = incense_type.incense_points as u64;
    let merit = incense_type.merit as u64;

    // Check daily limit
    ctx.accounts
        .user_incense_state
        .check_daily_incense_limit(incense_id, amount as u8)?;

    if amount == 0 {
        return err!(ErrorCode::InvalidAmount);
    }

    let fee_per_incense = ctx.accounts.temple_config.get_fee_per_incense(incense_id);
    let total_fee = fee_per_incense
        .checked_mul(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // enough balance
    if ctx.accounts.authority.lamports() < total_fee {
        return err!(ErrorCode::InsufficientSolBalance);
    }

    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.temple_treasury.to_account_info(),
            },
        ),
        total_fee,
    )?;

    msg!(
        "User bought {} of incense type {} (total fee: {} SOL)",
        amount,
        incense_id,
        total_fee as f64 / 1e9
    );

    // Mint NFT
    let temple_config_key: Pubkey = ctx.accounts.temple_config.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        IncenseNFT::SEED_PREFIX.as_bytes(),
        temple_config_key.as_ref(),
        &[incense_id],
        &[ctx.bumps.nft_mint_account],
    ]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.nft_mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )?;
    msg!("NFT minted successfully");

    // Immediately burn NFT
    burn(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        amount,
    )?;
    msg!("NFT burned successfully - consumable incense used");

    ctx.accounts.user_incense_state.incense_number += amount as u8;

    // Update user's incense points and merit
    ctx.accounts
        .user_incense_state
        .add_incense_value_and_merit(incense_points * amount, merit * amount);

    // Update global stats with merit and incense points
    ctx.accounts
        .global_stats
        .add_incense_value_and_merit(incense_points * amount, merit * amount);

    // Amulet drop logic - auto mint NFT
    let mut amulet_dropped = false;
    let mut amulet_type = String::new();

    // Generate random number for amulet drop
    let random_seed = current_time + ctx.accounts.authority.key().as_ref()[0] as u64;
    let random_value = (random_seed % 100) as u8;

    let mut dropped_amulet_type: u8 = 0;

    match incense_id {
        1 => {
            // 清香 (Clear Incense) - 5% chance for Fortune Amulet
            if random_value < 5 {
                amulet_dropped = true;
                dropped_amulet_type = 0; // Fortune Amulet
                msg!("Congratulations! Obtained Fortune Amulet from burning Clear Incense!");
            }
        }
        5 => {
            // 太上灵香 (Supreme Spirit Incense) - 10% chance for Merit Amulet
            if random_value < 10 {
                amulet_dropped = true;
                dropped_amulet_type = 2; // Merit Amulet
                msg!("Congratulations! Obtained Merit Amulet from burning Supreme Spirit Incense!");
            }
        }
        _ => {}
    }

    if amulet_dropped {
        // Emit amulet dropped event with type information
        emit!(crate::state::event::AmuletDropped {
            user: ctx.accounts.authority.key(),
            amulet_type: dropped_amulet_type,
            source: format!("burn_incense_{}", incense_id),
            timestamp: clock.unix_timestamp,
        });
    }

    // event
    emit!(IncenseBurned {
        user: ctx.accounts.authority.key(),
        incense_id: incense_id,
        amount: amount,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8)]
pub struct BurnIncense<'info> {
    /// User account (payer, signer)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: Temple admin account
    #[account(mut,
        constraint = temple_authority.key() == temple_config.owner @ ErrorCode::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    /// CHECK: Temple SOL storage account
    #[account(
        mut,
        constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury
    )]
    pub temple_treasury: AccountInfo<'info>,

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

    /// User account
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    /// User incense state
    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    /// NFT mint
    #[account(
        mut,
        seeds = [IncenseNFT::SEED_PREFIX.as_bytes(), temple_config.key().as_ref(), &[incense_id]],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_authority.key(), // Temple has freeze authority
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// User's NFT associated account
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
