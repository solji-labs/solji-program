use crate::error::ErrorCode;
use crate::state::amulet::{AmuletNFT, AmuletSource};
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::UserState;
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

// TODO Draw fortune/make wish - can be obtained probabilistically? Handle probability on frontend?
pub fn mint_amulet_nft(ctx: Context<MintAmuletNFT>, source: u8) -> Result<()> {
    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp as u64;

    // Check temple status
    ctx.accounts.temple_config.can_perform_operation(
        crate::state::temple_config::TempleStatusBitIndex::MintNFT,
        current_time,
    )?;

    // Check if user has sufficient pending_amulets balance
    require!(
        ctx.accounts.user_state.pending_amulets > 0,
        crate::error::ErrorCode::InsufficientPendingAmulets
    );

    // Consume one pending_amulet
    ctx.accounts.user_state.pending_amulets -= 1;

    // Get serial number (increment total_amulets first)
    ctx.accounts.user_state.total_amulets += 1;
    let serial_number: u32 = ctx.accounts.user_state.total_amulets;

    let nft_name_str = format!("Amulet #{}", serial_number);
    let source_str = match source {
        0 => "fortune",
        1 => "wish",
        _ => "unknown",
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
                update_authority: ctx.accounts.temple_config.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            temple_signer_seeds,
        ),
        DataV2 {
            name: nft_name_str.clone(),
            symbol: "AMULET".to_string(),
            uri: format!(
                "https://api.foxverse.co/temple/amulet/{}/metadata.json",
                serial_number
            ),
            seller_fee_basis_points: 500, // 5% royalty
            creators: None,
            collection: None,
            uses: None,
        },
        false, // immutable
        true,
        None,
    )?;

    // Mint amulet NFT
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
    msg!("Amulet NFT minted successfully");

    // Update temple config
    ctx.accounts.temple_config.total_amulets += 1;

    // Update global stats
    ctx.accounts.global_stats.increment_amulets();

    // Emit amulet minted event
    emit!(crate::state::event::AmuletMinted {
        user: ctx.accounts.authority.key(),
        amulet_mint: ctx.accounts.nft_mint_account.key(),
        source: source_str.to_string(),
        serial_number,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct MintAmuletNFT<'info> {
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

    /// User account
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        init,
        payer = authority,
        seeds = [
            AmuletNFT::SEED_PREFIX.as_bytes(),
            authority.key().as_ref(),
            &format!("{}", user_state.total_amulets).as_bytes(), 
        ],
        bump,
        mint::decimals = AmuletNFT::TOKEN_DECIMALS,
        mint::authority = temple_config.key(),
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// User's NFT associated token account
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    /// CHECK： Metadata account
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
