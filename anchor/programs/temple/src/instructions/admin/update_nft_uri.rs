use crate::incense_nft::IncenseNFT;
use crate::state::temple_config::*;
use anchor_lang::prelude::*;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::update_metadata_accounts_v2;
use anchor_spl::metadata::Metadata;
use anchor_spl::metadata::UpdateMetadataAccountsV2;
use anchor_spl::token::Mint;

pub fn update_nft_uri(ctx: Context<UpdateNftUri>, incense_id: u8, new_uri: String) -> Result<()> {
    let incense_type = ctx
        .accounts
        .temple_config
        .find_incense_type(incense_id)
        .ok_or(crate::error::ErrorCode::InvalidIncenseId)?;

    let data_v2 = DataV2 {
        name: format!("{} NFT", incense_type.name),
        symbol: IncenseNFT::TOKEN_SYMBOL.to_string(),
        uri: new_uri,
        seller_fee_basis_points: 0,
        creators: None,
        collection: None,
        uses: None,
    };

    let signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    update_metadata_accounts_v2(
        CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.meta_account.to_account_info(),
                update_authority: ctx.accounts.temple_authority.to_account_info(),
            },
            signer_seeds,
        ),
        None,
        Some(data_v2),
        None,
        None,
    )?;

    msg!(
        "NFT metadata URI updated successfully for incense type: {}",
        incense_id
    );
    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8, new_uri: String)]
pub struct UpdateNftUri<'info> {
    pub admin_authority: Signer<'info>,

    /// CHECK: This is the temple authority account that matches temple_config.owner
    #[account(mut,
        constraint = temple_authority.key() == temple_config.owner @ crate::error::ErrorCode::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    /// NFT Mint Account
    #[account(
        seeds = [
            IncenseNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            &[incense_id]
        ],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        // Note: We don't check freeze authority here as it's not required for metadata updates
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// CHECK: This is the Metaplex Token Metadata PDA derived from the NFT mint.
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

    /// Metaplex Token Metadata Program
    pub token_metadata_program: Program<'info, Metadata>,
}
