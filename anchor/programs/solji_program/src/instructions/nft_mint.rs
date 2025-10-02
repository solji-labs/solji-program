use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{create_master_edition_v3, create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMasterEditionV3, CreateMetadataAccountsV3, Metadata}, token::{ mint_to,  Mint, MintTo, Token, TokenAccount}};

use crate::{ events::SbtMintedEvent, states::{create_master_edition, create_metadata, creatre_freeze_account, mint_nft, CreateNftArgs, NftAccounts, SbtNftCount, UserInfo}};
// deprecated
pub fn nft_mint(ctx: Context<CreateBurnToken>, args: CreateNftArgs) -> Result<()> {
  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_burn_token",
      args.name.as_bytes(),
      &[ctx.bumps.nft_mint_account],
    ]];  
   create_metadata_accounts_v3(CpiContext::new_with_signer(
    ctx.accounts.token_metadata_program.to_account_info(), 
    CreateMetadataAccountsV3 {
        metadata: ctx.accounts.metadata_account.to_account_info(),
        mint: ctx.accounts.nft_mint_account.to_account_info(),
        mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
        payer: ctx.accounts.authority.to_account_info() ,
        update_authority:ctx.accounts.nft_mint_account.to_account_info(),
        system_program:ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
    },
     signer_seeds), 
     DataV2{
       name: args.name.to_string(),
        symbol: args.symbol.to_string(), 
        uri: args.url.to_string(), 
        seller_fee_basis_points: 0, 
        creators: None, collection: None, uses:None 
      }, 
      false, 
      true,
       None)?;

       mint_to(CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(),
        MintTo{
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            to:ctx.accounts.nft_associated_token_account.to_account_info(),
            authority: ctx.accounts.nft_mint_account.to_account_info(),
        },
         signer_seeds),
         1)?;

         msg!("nft mint success ata:{}",ctx.accounts.nft_associated_token_account.key());

         create_master_edition_v3(CpiContext::new_with_signer(
          ctx.accounts.token_metadata_program.to_account_info(), 
          CreateMasterEditionV3{
            edition: ctx.accounts.master_editon_account.to_account_info(),
            mint: ctx.accounts.nft_mint_account.to_account_info(),
            update_authority: ctx.accounts.nft_mint_account.to_account_info(),
            mint_authority: ctx.accounts.nft_mint_account.to_account_info(),
            payer: ctx.accounts.authority.to_account_info(),
            metadata: ctx.accounts.metadata_account.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
        }, 
          signer_seeds), 
          Some(1))?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(args: CreateNftArgs)]
pub struct CreateBurnToken<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
       init,
       payer = authority, 
       seeds = [b"create_burn_token",args.name.as_bytes()],
       mint::decimals = 0,
       mint::authority = nft_mint_account,
       mint::freeze_authority = nft_mint_account,
       bump,
      )]
    pub nft_mint_account: Account<'info, Mint>,

    /// CHECK:
    #[account(
      mut,
      seeds = [b"metadata",token_metadata_program.key().as_ref(),nft_mint_account.key().as_ref(),  b"edition".as_ref(),],
      bump,
      seeds::program = token_metadata_program.key(),
    )]
    pub master_editon_account:UncheckedAccount<'info>,


    ///CHECK:
    #[account(
      mut,
      seeds = [b"metadata",token_metadata_program.key().as_ref(),nft_mint_account.key().as_ref()],
      bump,
      seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,


    #[account(
      init_if_needed,
      payer = authority,
      associated_token::mint = nft_mint_account,
      associated_token::authority = authority,
    )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}


pub fn mint_sbt_nft(ctx: Context<MintSbtNft>, args: CreateNftArgs) -> Result<()> {
  require!(ctx.accounts.sbt_nft_count.count <= 10_000, NftErrorCode::NftCountOver);
  {
      let ui = &ctx.accounts.user_info; 
      require!(!ui.has_sbt_token, NftErrorCode::HasSbtToken);
      require!(ui.donate_amount >= 500_000_000, NftErrorCode::DonateAmountNotEnough);
  }
  
  let name_bytes =  args.name.as_bytes();
  let authority = ctx.accounts.authority.key();
  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_sbt_token",
      authority.as_ref(),
      name_bytes,
      &[ctx.bumps.sbt_nft_mint_account],
  ]];
  
  // mint nft 
  let accounts = NftAccounts {
    token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
    metadata_account: ctx.accounts.metadata_account.to_account_info(),
    nft_mint_account: ctx.accounts.sbt_nft_mint_account.to_account_info(),
    authority: ctx.accounts.authority.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
    token_program: ctx.accounts.token_program.to_account_info(),
    nft_associated_token_account: ctx.accounts.sbt_nft_associated_token_account.to_account_info(),
    master_edition_account: ctx.accounts.master_editon_account.to_account_info(),
  };

  create_metadata(&accounts, CreateNftArgs{
    name:args.name.clone(),
    symbol: args.symbol.clone(),
    url: args.url.clone(),
    is_mutable: args.is_mutable,
    collection_details: args.collection_details,
  }, signer_seeds)?;

  mint_nft(&accounts,signer_seeds)?;

  creatre_freeze_account(&accounts, signer_seeds)?;

  create_master_edition(&accounts,signer_seeds)?;

  msg!("SBT mint success ata: {}", ctx.accounts.sbt_nft_associated_token_account.key());

  {
    ctx.accounts.sbt_nft_count.increment()?;
  }
  
  {
    ctx.accounts.user_info.has_sbt_token = true;
  }
  
  let now = Clock::get()?.unix_timestamp;
  emit!(SbtMintedEvent {
      authority,
      mint: ctx.accounts.sbt_nft_mint_account.key(),
      ata: ctx.accounts.sbt_nft_associated_token_account.key(),
      name: args.name.clone(),
      symbol: args.symbol.clone(),
      url: args.url.clone(),
      donate_amount: ctx.accounts.user_info.donate_amount,
      timestamp: now,
  });

  Ok(())
}



#[derive(Accounts)]
#[instruction(args: CreateNftArgs)]
pub struct MintSbtNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /*
     * SBT NFT created by administrator for user payer ->administrator
     * Users can mint their own SBT NFT payer ->Users
     */
    #[account(mut)]
    pub payer:Signer<'info>,
  
    #[account(
       init,
       payer = payer, 
       seeds = [b"create_sbt_token",authority.key().as_ref(), args.name.as_bytes()],
       mint::decimals = 0,
       mint::authority = sbt_nft_mint_account,
       mint::freeze_authority = sbt_nft_mint_account,
       bump,
    )]
    pub sbt_nft_mint_account: Account<'info, Mint>,

    /// CHECK:
    #[account(
      mut,
      seeds = [b"metadata", token_metadata_program.key().as_ref(), sbt_nft_mint_account.key().as_ref()],
      bump,
      seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK:
    #[account(
      mut,
      seeds = [b"metadata", token_metadata_program.key().as_ref(), sbt_nft_mint_account.key().as_ref(), b"edition".as_ref()],
      bump,
      seeds::program = token_metadata_program.key(),
    )]
    pub master_editon_account: UncheckedAccount<'info>,

    #[account(
      init_if_needed,
      payer = payer,
      associated_token::mint = sbt_nft_mint_account,
      associated_token::authority = authority,
    )]
    pub sbt_nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(
      init_if_needed,
      payer = authority,
      seeds = ["sbt_nft_count".as_bytes(),],
      bump,
      space = 8 + SbtNftCount::INIT_SPACE,
    )]
    pub sbt_nft_count: Account<'info, SbtNftCount>,

    #[account(
      mut,
      seeds = [b"user_info",authority.key().as_ref()],
      bump
    )]
    pub user_info: Account<'info, UserInfo>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum NftErrorCode{
  #[msg("NftCountOver")]
  NftCountOver,

  #[msg("Already Have Sbt Nft")]
  HasSbtToken,

  #[msg("Donate Amount Not Enough")]
  DonateAmountNotEnough,
}