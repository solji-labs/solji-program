use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata::{ Metadata}, token::{  Mint, Token, TokenAccount}};

use crate::{ events::SbtMintedEvent, global_error::GlobalError, states::{create_metadata, creatre_freeze_account, mint_nft, AmuletLevel, CreateNftArgs, IncenseType, LotteryRecord, NftAccounts, SbtNftCount, Temple, UserInfo, WishUser, SBT_NFT_NAME, SBT_NFT_SYMBOL, SBT_NFT_URL}};
pub fn burn_incense_nft_mint(ctx: Context<CreateBurnToken>, incense: u8) -> Result<()> {

  let incense_type =  IncenseType::get_incense_type(incense).ok_or(GlobalError::InvalidIncenseType)?;

  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_burn_token",
      &[incense],
      &[ctx.bumps.burn_nft_mint_account],
    ]];  

    let accounts = NftAccounts {
      token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
      metadata_account: ctx.accounts.metadata_account.to_account_info(),
      nft_mint_account: ctx.accounts.burn_nft_mint_account.to_account_info(),
      payer: ctx.accounts.admin.to_account_info(),
      system_program: ctx.accounts.system_program.to_account_info(),
      rent: ctx.accounts.rent.to_account_info(),
      token_program: ctx.accounts.token_program.to_account_info(),
      nft_associated_token_account: None,
      master_edition_account: None,
  };
  

  create_metadata(&accounts, CreateNftArgs{
      name:incense_type.get_nft_name(),
      symbol: incense_type.get_symbol(),
      url: incense_type.get_nft_uri(),
      is_mutable: false,
      collection_details: true,
  }, signer_seeds)?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(incense: u8)]
pub struct CreateBurnToken<'info> {

    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
      mut,
      seeds = [b"temple"],
      bump,
      has_one = admin,
  )]
  pub temple: Account<'info, Temple>,

    #[account(
      init,
      payer = admin, 
      seeds = [b"create_burn_token".as_ref(),&[incense]],
      mint::decimals = 0,
      mint::authority = burn_nft_mint_account,
      mint::freeze_authority = burn_nft_mint_account,
      bump,
     )]
   pub burn_nft_mint_account: Account<'info, Mint>,

    ///CHECK:
    #[account(
      mut,
      seeds = [b"metadata", token_metadata_program.key().as_ref(),burn_nft_mint_account.key().as_ref()],
      bump,
      seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub rent: Sysvar<'info, Rent>,
}


pub fn mint_sbt_nft(ctx: Context<MintSbtNft>) -> Result<()> {
  require!(ctx.accounts.sbt_nft_count.count <= 10_000, NftErrorCode::NftCountOver);
  {
      let ui = &ctx.accounts.user_info; 
      require!(!ui.has_sbt_token, NftErrorCode::HasSbtToken);
      require!(ui.donate_amount >= 500_000_000, NftErrorCode::DonateAmountNotEnough);
  }
  
  let authority = ctx.accounts.authority.key();
  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_sbt_token",
      authority.as_ref(),
      &[ctx.bumps.sbt_nft_mint_account],
  ]];
  
  // mint nft 
  let accounts = NftAccounts {
    token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
    metadata_account: ctx.accounts.metadata_account.to_account_info(),
    nft_mint_account: ctx.accounts.sbt_nft_mint_account.to_account_info(),
    payer: ctx.accounts.authority.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
    token_program: ctx.accounts.token_program.to_account_info(),
    nft_associated_token_account: Some(ctx.accounts.sbt_nft_associated_token_account.to_account_info()),
    master_edition_account: None,
  };

  let name =  format!("{}#{}",SBT_NFT_NAME,ctx.accounts.sbt_nft_count.count+1);

  create_metadata(&accounts, CreateNftArgs{
    name: name.clone(),
    symbol: SBT_NFT_SYMBOL.to_string(),
    url: SBT_NFT_URL.to_string(),
    is_mutable: false,
    collection_details: true,
  }, signer_seeds)?;

  mint_nft(&accounts,signer_seeds)?;

  creatre_freeze_account(&accounts, signer_seeds)?;

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
      name: name.clone(),
      symbol: SBT_NFT_SYMBOL.to_string(),
      url:  SBT_NFT_URL.to_string(),
      donate_amount: ctx.accounts.user_info.donate_amount,
      timestamp: now,
  });

  Ok(())
}

#[derive(Accounts)]
pub struct MintSbtNft<'info> {

    #[account(mut)]
    pub authority: Signer<'info>,

  
    #[account(
       init,
       payer = authority, 
       seeds = [b"create_sbt_token",authority.key().as_ref()],
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


    #[account(
      init_if_needed,
      payer = authority,
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


pub fn draw_mint_nft(ctx: Context<DrawMintNft>) -> Result<()> { 
  let accounts = NftAccounts {
    token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
    metadata_account: ctx.accounts.metadata_account.to_account_info(),
    nft_mint_account: ctx.accounts.draw_nft_mint_account.to_account_info(),
    payer: ctx.accounts.admin.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
    token_program: ctx.accounts.token_program.to_account_info(),
    nft_associated_token_account:None,
    master_edition_account: None,
  };

  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_draw_token",
      &[ctx.bumps.draw_nft_mint_account],
      ]];

  create_metadata(&accounts, CreateNftArgs{
      name: LotteryRecord::NAME.to_string(),
      symbol: LotteryRecord::SYMBOL.to_string(),
      url: LotteryRecord::URL.to_string(),
      is_mutable: false,
      collection_details: true,
    }, signer_seeds)?;

  Ok(())
}


#[derive(Accounts)]
pub struct DrawMintNft<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump,
        has_one = admin,
    )]
    pub temple: Account<'info, Temple>,


    #[account(
        init,
        payer = admin, 
        seeds = [b"create_draw_token"],
        mint::decimals = 0,
        mint::authority = draw_nft_mint_account,
        mint::freeze_authority = draw_nft_mint_account,
        bump,
     )]
     pub draw_nft_mint_account: Account<'info, Mint>,
 
     /// CHECK:
     #[account(
       mut,
       seeds = [b"metadata", token_metadata_program.key().as_ref(), draw_nft_mint_account.key().as_ref()],
       bump,
       seeds::program = token_metadata_program.key(),
     )]
     pub metadata_account: UncheckedAccount<'info>,
 
     pub system_program: Program<'info, System>,
     pub token_program: Program<'info, Token>,
     pub token_metadata_program: Program<'info, Metadata>,
     pub associated_token_program: Program<'info, AssociatedToken>,
     pub rent: Sysvar<'info, Rent>,
}


pub fn wish_mint_nft(ctx: Context<WishMintNft>) -> Result<()> { 
  let accounts = NftAccounts {
    token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
    metadata_account: ctx.accounts.metadata_account.to_account_info(),
    nft_mint_account: ctx.accounts.wish_nft_mint_account.to_account_info(),
    payer: ctx.accounts.admin.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
    token_program: ctx.accounts.token_program.to_account_info(),
    nft_associated_token_account:None,
    master_edition_account: None,
  };

  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_wish_token",
      &[ctx.bumps.wish_nft_mint_account],
      ]];

  create_metadata(&accounts, CreateNftArgs{
      name: WishUser::NAME.to_string(),
      symbol: WishUser::SYMBOL.to_string(),
      url: WishUser::URL.to_string(),
      is_mutable: false,
      collection_details: true,
    }, signer_seeds)?;

  Ok(())
}


#[derive(Accounts)]
pub struct WishMintNft<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump,
        has_one = admin,
    )]
    pub temple: Account<'info, Temple>,


    #[account(
        init,
        payer = admin, 
        seeds = [b"create_wish_token"],
        mint::decimals = 0,
        mint::authority = wish_nft_mint_account,
        mint::freeze_authority = wish_nft_mint_account,
        bump,
     )]
     pub wish_nft_mint_account: Account<'info, Mint>,
 
     /// CHECK:
     #[account(
       mut,
       seeds = [b"metadata", token_metadata_program.key().as_ref(), wish_nft_mint_account.key().as_ref()],
       bump,
       seeds::program = token_metadata_program.key(),
     )]
     pub metadata_account: UncheckedAccount<'info>,
 
     pub system_program: Program<'info, System>,
     pub token_program: Program<'info, Token>,
     pub token_metadata_program: Program<'info, Metadata>,
     pub associated_token_program: Program<'info, AssociatedToken>,
     pub rent: Sysvar<'info, Rent>,
}


pub fn amulet_mint_nft(ctx: Context<AmuletMintNft>,amulet:u8) -> Result<()> { 
  let amulet_level = AmuletLevel::try_from(amulet)?; 
  let meta = amulet_level.meta();

  let accounts = NftAccounts {
    token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
    metadata_account: ctx.accounts.metadata_account.to_account_info(),
    nft_mint_account: ctx.accounts.amulet_nft_mint_account.to_account_info(),
    payer: ctx.accounts.admin.to_account_info(),
    system_program: ctx.accounts.system_program.to_account_info(),
    rent: ctx.accounts.rent.to_account_info(),
    token_program: ctx.accounts.token_program.to_account_info(),
    nft_associated_token_account:None,
    master_edition_account: None,
  };

  let signer_seeds: &[&[&[u8]]] = &[&[
      b"create_amulet_token",
      &[amulet],
      &[ctx.bumps.amulet_nft_mint_account],
      ]];

  create_metadata(&accounts, CreateNftArgs{
      name: meta.name.to_string(),
      symbol: meta.symbol.to_string(),
      url: meta.uri.to_string(),
      is_mutable: false,
      collection_details: true,
    }, signer_seeds)?;

  Ok(())
}


#[derive(Accounts)]
#[instruction(amulet: u8)]
pub struct AmuletMintNft<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump,
        has_one = admin,
    )]
    pub temple: Account<'info, Temple>,


    #[account(
        init,
        payer = admin, 
        seeds = [b"create_amulet_token".as_ref(),&[amulet]],
        mint::decimals = 0,
        mint::authority = amulet_nft_mint_account,
        mint::freeze_authority = amulet_nft_mint_account,
        bump,
     )]
     pub amulet_nft_mint_account: Account<'info, Mint>,
 
     /// CHECK:
     #[account(
       mut,
       seeds = [b"metadata", token_metadata_program.key().as_ref(), amulet_nft_mint_account.key().as_ref()],
       bump,
       seeds::program = token_metadata_program.key(),
     )]
     pub metadata_account: UncheckedAccount<'info>,
 
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

