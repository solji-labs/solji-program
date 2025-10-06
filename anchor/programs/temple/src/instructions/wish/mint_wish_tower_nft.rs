use crate::error::ErrorCode;
use crate::state::event::WishTowerNFTMinted;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::*;
use crate::state::wish_tower::*;
use crate::state::wish_tower_nft::*;
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

pub fn mint_wish_tower_nft(ctx: Context<MintWishTowerNFT>) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::MintNFT,
        current_time,
    )?;

    // No completion requirement - can mint at any time

    // Check if user owns the tower
    require!(
        ctx.accounts.wish_tower_account.creator == ctx.accounts.authority.key(),
        ErrorCode::InvalidOwner
    );

    // Check if NFT already minted for this tower
    require!(
        ctx.accounts.wish_tower_nft_account.mint == Pubkey::default(),
        ErrorCode::WishAlreadyExists
    );

    // Mint Wish Tower NFT
    let wish_count = ctx.accounts.wish_tower_account.wish_count;
    let level = ctx.accounts.wish_tower_account.level;

    let nft_name = format!("Wish Tower ({} wishes, Level {})", wish_count, level);
    let nft_uri = format!(
        "https://example.com/wish-tower/{}/metadata.json",
        ctx.accounts.authority.key()
    );

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
            name: nft_name,
            symbol: "WISH_TOWER".to_string(),
            uri: nft_uri,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, // immutable
        true,
        None,
    )?;

    // Mint Wish Tower NFT token
    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

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
    msg!("Wish Tower NFT minted successfully");

    // Initialize WishTowerNFT account data
    ctx.accounts.wish_tower_nft_account.owner = ctx.accounts.authority.key();
    ctx.accounts.wish_tower_nft_account.mint = ctx.accounts.nft_mint_account.key();
    ctx.accounts.wish_tower_nft_account.tower_id = 0; // Not used in simplified version
    ctx.accounts.wish_tower_nft_account.level = level;
    ctx.accounts.wish_tower_nft_account.wish_count = wish_count as u8;
    ctx.accounts.wish_tower_nft_account.minted_at = clock.unix_timestamp;

    // Update global stats
    ctx.accounts.global_stats.increment_buddha_lights(); // Reuse this counter for wish tower NFTs

    // Emit event
    emit!(WishTowerNFTMinted {
        user: ctx.accounts.authority.key(),
        nft_mint: ctx.accounts.nft_mint_account.key(),
        wish_count,
        level,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct MintWishTowerNFT<'info> {
    /// User account
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        seeds = [WishTower::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump = wish_tower_account.bump,
    )]
    pub wish_tower_account: Box<Account<'info, WishTower>>,

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
        init,
        payer = authority,
        seeds = [
            b"WishTowerNFT",
            wish_tower_account.key().as_ref(),
        ],
        bump,
        mint::decimals = 0,
        mint::authority = temple_config.key(),
        mint::freeze_authority = temple_config.key(),
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// User's NFT associated token account
    #[account(
        init,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        space = 8 + WishTowerNFT::INIT_SPACE,
        seeds = [b"WishTowerNFTAccount", wish_tower_account.key().as_ref()],
        bump
    )]
    pub wish_tower_nft_account: Account<'info, WishTowerNFT>,

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
