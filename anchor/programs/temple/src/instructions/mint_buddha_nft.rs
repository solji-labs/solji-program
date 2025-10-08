use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::freeze_account;
use anchor_spl::token::mint_to;
use anchor_spl::token::FreezeAccount;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

use crate::state::{BuddhaNft, TempleError, TempleConfig, UserState, UserDonationState};
use crate::BuddhaNftError;


pub fn mint_buddha_nft(ctx: Context<MintBuddhaNft>) -> Result<()> {

    let clock = Clock::get()?;
    let current_time = clock.unix_timestamp;

    require!(
        ctx.accounts.temple_config.total_buddha_nft < 10000,
        TempleError::BuddhaNftCountOverflow,
    );

    require!(
        ctx.accounts.user_donation_state.can_mint_buddha_nft(),
        BuddhaNftError::UserCannotMintBuddhaNft,
    );

    require!(
        ctx.accounts.user_donation_state.has_minted_buddha_nft(),
        BuddhaNftError::UserAlreadyMintedBuddhaNft,
    );


    let serial_number = ctx.accounts.temple_config.total_buddha_nft;

    let nft_name = format!("Buddha NFT #{}", serial_number);

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
                payer: ctx.accounts.user.to_account_info(),
                update_authority: ctx.accounts.temple_config.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            temple_signer_seeds,
        ),
        DataV2 {
            name: nft_name,
            symbol: BuddhaNft::TOKEN_SYMBOL.to_string(),
            uri: BuddhaNft::TOKEN_URI.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false, 
        true, 
        None,
    )?;

    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx.accounts.user_nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.temple_config.to_account_info(),
            },
            temple_signer_seeds,
        ), 
    1)?;


    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    freeze_account(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            FreezeAccount {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                account: ctx.accounts.user_nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.temple_config.to_account_info(),
            },
            temple_signer_seeds,
        ),
    )?;





    let buddha_nft_account = &mut ctx.accounts.buddha_nft_account;
    buddha_nft_account.initialize(
        ctx.accounts.user.key(),
        ctx.accounts.nft_mint_account.key(),
        serial_number,
        current_time,
    );

    let user_donation_state = &mut ctx.accounts.user_donation_state;
    user_donation_state.mint_buddha_nft()?;


    let temple_config = &mut ctx.accounts.temple_config;
    temple_config.mint_buddha_nft()?;


    Ok(())
}


#[derive(Accounts)]
pub struct MintBuddhaNft<'info> {


    #[account(
        init,
        payer = user,
        space = 8 + BuddhaNft::INIT_SPACE,
        seeds = [
            BuddhaNft::SEED_PREFIX.as_bytes(),
            b"account",
            temple_config.key().as_ref(),
               user.key().as_ref(),
        ],
        bump,
    )]
    pub buddha_nft_account: Account<'info, BuddhaNft>,

     /// 用户状态账户
     #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    /// 用户捐助状态账户
    #[account(
        mut,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Account<'info, UserDonationState>,

     
    #[account(mut)]
    pub user: Signer<'info>,

    /// 寺庙状态账户
    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Account<'info, TempleConfig>,

    /// nft mint
    #[account(
        init_if_needed,
        payer = user,
        seeds = [
            BuddhaNft::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
               user.key().as_ref(),
        ],
        bump,   
        mint::decimals = BuddhaNft::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_config.key(),
    )]
    pub nft_mint_account: Account<'info, Mint>,


        /// 用户NFT关联账户
        #[account(
            init_if_needed,
            payer = user,
            associated_token::mint = nft_mint_account,
            associated_token::authority = user,
        )]
        pub user_nft_associated_token_account: Account<'info, TokenAccount>,

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

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
 }