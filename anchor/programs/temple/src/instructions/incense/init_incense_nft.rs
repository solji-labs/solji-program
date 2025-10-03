use crate::state::*;
use anchor_lang::prelude::*;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::{Mint, Token};

pub fn init_incense_nft(ctx: Context<InitIncenseNft>, incense_type_id: u8) -> Result<()> {
    let incense_type_config = &mut ctx.accounts.incense_type_config;

    let nft_name = format!("{} NFT", incense_type_config.name);

    let temple_state_key = &mut ctx.accounts.temple_state.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        IncenseNFT::SEED_PREFIX.as_bytes(),
        temple_state_key.as_ref(),
        &[incense_type_id],
        &[ctx.bumps.nft_mint_account],
    ]];

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                update_authority: ctx.accounts.temple_authority.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
            signer_seeds,
        ),
        DataV2 {
            name: nft_name,
            symbol: IncenseNFT::TOKEN_SYMBOL.to_string(),
            uri: IncenseNFT::TOKEN_URL.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        false,
        true,
        None,
    );

    msg!("Incense NFT initialized successfully");

    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_type_id :u8)]
pub struct InitIncenseNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 香型配置账户
    #[account(
        mut,
        seeds = [
            IncenseTypeConfig::SEED_PREFIX.as_bytes(),
             &[incense_type_id]
             ],
        bump,
    )]
    pub incense_type_config: Account<'info, IncenseTypeConfig>,

    /// CHECK: 寺庙管理员账号
    #[account(mut,
        constraint = temple_authority.key() == temple_state.authority @ BurnIncenseError::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    /// 寺庙状态账户
    #[account(
        mut,
        seeds = [TempleState::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_state: Account<'info, TempleState>,

    /// nft mint
    #[account(
        init_if_needed,
        payer = authority,
        seeds = [
            IncenseNFT::SEED_PREFIX.as_bytes(),
               temple_state.key().as_ref(),
               &[incense_type_id]],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_authority.key(),
    )]
    pub nft_mint_account: Account<'info, Mint>,

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

    pub rent: Sysvar<'info, Rent>,
}
