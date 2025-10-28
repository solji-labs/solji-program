use crate::incense_nft::IncenseNFT;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

pub fn mint_nft_to_user(ctx: Context<MintNftToUser>, incense_id: u8) -> Result<()> {
    // Only allow temple owner to mint NFTs
    require!(
        ctx.accounts.temple_config.owner == ctx.accounts.authority.key(),
        crate::error::ErrorCode::InvalidOwner
    );

    // 1. Fetch incense configuration
    let _incense_type = ctx
        .accounts
        .temple_config
        .find_incense_type(incense_id)
        .ok_or(crate::error::ErrorCode::InvalidIncenseId)?;

    // 2. Mint NFT to user's associated token account
    // Use the NFT mint account itself as the authority (it has mint authority)
    let temple_config_key: Pubkey = ctx.accounts.temple_config.key();
    let mint_signer_seeds: &[&[&[u8]]] = &[&[
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
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.nft_mint_account.to_account_info(), // Mint account is its own authority
            },
            mint_signer_seeds,
        ),
        1, // Mint exactly 1 NFT
    )?;

    msg!(
        "Minted incense NFT type {} to user: {}",
        incense_id,
        ctx.accounts.user.key()
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8)]
pub struct MintNftToUser<'info> {
    /// The admin wallet signing the transaction (must be temple owner)
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This is the temple authority account that matches temple_config.owner
    #[account(mut,
        constraint = temple_authority.key() == temple_config.owner @ crate::error::ErrorCode::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    /// Temple Configuration Account - This PDA is the CPI Signer, must be 'mut'.
    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    /// NFT Mint Account
    #[account(
        mut,
        seeds = [
            IncenseNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            &[incense_id]
        ],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(), // Mint authority is the mint itself
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// User's associated token account for the NFT
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = user,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    /// The user receiving the NFT
    /// CHECK: This is the user account
    pub user: AccountInfo<'info>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
