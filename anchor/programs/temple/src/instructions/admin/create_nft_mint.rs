use crate::incense_nft::IncenseNFT;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;
use anchor_spl::metadata::create_metadata_accounts_v3;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::CreateMetadataAccountsV3;
use anchor_spl::metadata::Metadata;
use anchor_spl::token::{Mint, Token};

pub fn create_nft_mint(ctx: Context<CreateNftMint>, incense_id: u8) -> Result<()> {
    // Incense Conformation
    let incense_type = ctx
        .accounts
        .temple_config
        .find_incense_type(incense_id)
        .ok_or(crate::error::ErrorCode::InvalidIncenseId)?;

    let nft_name = format!("{} NFT", incense_type.name);

    // 为每个香型设置不同的元数据URL
    let metadata_uri = match incense_id {
        1 => "https://solji.mypinata.cloud/ipfs/QmfE3pH44ef4iHHS7Vv81aDomY7yTzUtPnKxcBtZXyMkh4", 
        2 => "https://solji.mypinata.cloud/ipfs/QmYBz666XhqdQtizZYgg4C6EH3cKKKDPRdNDZZ4SEcAxDD",
        3 => "https://solji.mypinata.cloud/ipfs/QmUxi64HN4JZh11nztj7mQ3mnwKnadmuoStWR9cfkEqKNo",
        4 => "https://solji.mypinata.cloud/ipfs/QmPieVQDrCXs2hCB8SxpKGc3Rnh32M1eGCrjY4EbqguXQM",
        5 => "https://solji.mypinata.cloud/ipfs/bafkreiesfvlpyunybdl22oogzj2kxaado3hpblwqmd3q45pcu4imyzj3ha",
        6 => "https://solji.mypinata.cloud/ipfs/bafkreiesfvlpyunybdl22oogzj2kxaado3hpblwqmd3q45pcu4imyzj3ha",
        _ => "https://solji.mypinata.cloud/ipfs/QmfE3pH44ef4iHHS7Vv81aDomY7yTzUtPnKxcBtZXyMkh4",
    };

    let temple_config_key = ctx.accounts.temple_config.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        IncenseNFT::SEED_PREFIX.as_bytes(),
        temple_config_key.as_ref(),
        &[incense_id],
        &[ctx.bumps.nft_mint_account],
    ]];

    create_metadata_accounts_v3(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            CreateMetadataAccountsV3 {
                metadata: ctx.accounts.meta_account.to_account_info(),
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
                update_authority: ctx.accounts.temple_authority.to_account_info(),
                payer: ctx.accounts.authority.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
                rent: ctx.accounts.rent.to_account_info(),
            },
            signer_seeds,
        ),
        DataV2 {
            name: nft_name,
            symbol: IncenseNFT::TOKEN_SYMBOL.to_string(),
            uri: metadata_uri.to_string(),
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        },
        true,
        true,
        None,
    )?;

    msg!(
        "NFT mint created successfully for incense type: {}",
        incense_id
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8)]
pub struct CreateNftMint<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This is a temple authority account
    #[account(mut,
        constraint = temple_authority.key() == temple_config.owner @ crate::error::ErrorCode::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    /// nft mint
    #[account(
        init_if_needed,
        payer = authority,
        seeds = [
            IncenseNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            &[incense_id]
        ],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_authority.key(),
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

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

    /// CHECK: this is the master edition account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
            b"edition".as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub master_edition_account: UncheckedAccount<'info>,

    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
