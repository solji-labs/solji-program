use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};
use anchor_spl::{associated_token::AssociatedToken, metadata::{mpl_token_metadata::types::DataV2, update_metadata_accounts_v2, Metadata, UpdateMetadataAccountsV2}, token::{Mint, Token, TokenAccount}};
use crate::{ events::{DonateCountCreatedEvent, DonateEvent, MedalMintedEvent, MedalUpgradedEvent, UserActivityEvent}, states::{create_nft, donate, ActivityEnum, CreateNftArgs, DonateCounter, DonateRecord, MedalLevel, NftAccounts, Temple, UserInfo}};
pub fn create_donate_count(ctx: Context<CreateDonateCount>) -> Result<()> {
    let donate_count =  DonateCounter::new(ctx.accounts.authority.key());
    ctx.accounts.donate_count.set_inner(donate_count);

    emit!(DonateCountCreatedEvent {
        authority: ctx.accounts.authority.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}

#[derive(Accounts)]
pub struct CreateDonateCount<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + DonateCounter::INIT_SPACE, 
        seeds = [b"donate_count", authority.key().as_ref()],
        bump
    )]
    pub donate_count: Account<'info, DonateCounter>,


    #[account(
        init_if_needed,
        payer = authority, 
        seeds = [b"create_feats_nft",authority.key().as_ref()],
        mint::decimals = 0,
        mint::authority = feats_nft_mint_account,
        mint::freeze_authority = feats_nft_mint_account,
        bump,
    )]
    pub feats_nft_mint_account: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn create_donate_record(ctx: Context<CreateDonateRecord>, amount: u64) -> Result<()> {
    require!(amount > 0, DonateError::InvalidDonateAmount);
    require!(ctx.accounts.authority.to_account_info().lamports() >= amount, DonateError::InsufficientLamports);
    let authority_key = ctx.accounts.authority.key();
    let (merit_value, incense_value) = DonateRecord::get_donation_rewards(amount);

    let donate_record = DonateRecord::new(ctx.accounts.authority.key(), amount);
    ctx.accounts.donate_record.set_inner(donate_record);
    ctx.accounts.donate_count.increment()?;

    let user_info = &mut ctx.accounts.user_info;
    user_info.update_user_donate_amount(amount)?;   
    donate::calc_incense_rewards(amount, user_info)?;

    let level = DonateRecord::get_badge_level(user_info.donate_amount);

    ctx.accounts.temple.add_temple_donate_amount(amount)?;

    let tx = transfer(
        &ctx.accounts.authority.key(),
        &ctx.accounts.temple.key(),
        amount,
    );

    invoke(
        &tx,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.temple.to_account_info(),
        ],
    )?;
    msg!("Successful donation:{}",amount);
    
    emit!(DonateEvent {
        user: ctx.accounts.authority.key(),
        amount,
        merit_value,
        incense_value,
        timestamp: Clock::get()?.unix_timestamp,
    });

    let donate_amount: u64 = user_info.donate_amount;
    msg!("donate_amount: {}", donate_amount);

    if donate_amount < 50_000_000 {
        return Ok(());
    }

    let a = ctx.accounts.authority.key();
    let seeds: &[&[&[u8]]] = &[&[
        b"create_feats_nft",
        a.as_ref(),
        &[ctx.bumps.feats_nft_mint_account],
    ]];

    if matches!(Some(user_info.current_medal_level.clone()), Some(MedalLevel::None))
        && donate_amount > 50_000_000{
        emit!(MedalMintedEvent {
            user: authority_key,
            level: level.get_symbol(),
            nft_mint: ctx.accounts.feats_nft_mint_account.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        user_info.current_medal_level = level.clone();

        mint_nft(
            ctx,
            seeds,
            CreateNftArgs {
                name: level.get_nft_name(),
                symbol: level.get_symbol(),
                url: level.get_nft_uri(),
                is_mutable: true,
                collection_details: true,
            },
            merit_value,
            incense_value,
        )?;
    } else if let Some(current_medal_level) = Some(user_info.current_medal_level.clone()).as_ref() {
        if level != *current_medal_level {
            msg!(
                "Upgrade medal level:{}, existing grade:{}",
                level.get_nft_name(),
                current_medal_level.get_nft_name()
            );
            emit!(MedalUpgradedEvent {
                user: authority_key,
                old_level: current_medal_level.get_symbol(),
                new_level: level.get_symbol(),
                nft_mint: ctx.accounts.feats_nft_mint_account.key(),
                timestamp: Clock::get()?.unix_timestamp,
            });

            user_info.current_medal_level = level.clone();

            let data = DataV2 {
                name: level.get_nft_name(),
                symbol: level.get_symbol(),
                uri: level.get_nft_uri(),
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            };

            let cpi_ctx = CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                UpdateMetadataAccountsV2 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    update_authority: ctx.accounts.feats_nft_mint_account.to_account_info(),
                },
                seeds,
            );
            update_metadata_accounts_v2(
                cpi_ctx,
                None,          
                Some(data),
                Some(true),    
                Some(true),    
            )?;

            user_info.update_user_donate_info(merit_value, incense_value)?;
            DonateRecord::update_rewards(&mut ctx.accounts.donate_record, merit_value, incense_value);
        }
    }
    emit!(UserActivityEvent {
        user: authority_key,
        activity_type: ActivityEnum::Donate,
        content: amount.to_string(),
        timestamp: Clock::get()?.unix_timestamp,
    });
    Ok(())
}
pub fn mint_nft(
    ctx: Context<CreateDonateRecord>,
    seeds: &[&[&[u8]]],
    args:CreateNftArgs,
    merit_value:u64 ,
    incense_value: u64)-> Result<()> {
     // mint nft 
     let accounts = NftAccounts {
        token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
        metadata_account: ctx.accounts.metadata_account.to_account_info(),
        nft_mint_account: ctx.accounts.feats_nft_mint_account.to_account_info(),
        payer: ctx.accounts.authority.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        nft_associated_token_account: Some(ctx.accounts.user_receive_feats_nft_ata.to_account_info()),
        master_edition_account: Some(ctx.accounts.master_editon_account.to_account_info()),
    };

    create_nft(&accounts, args, seeds)?;

    UserInfo::update_user_donate_info(&mut ctx.accounts.user_info, merit_value, incense_value)?;

    DonateRecord::update_rewards(&mut ctx.accounts.donate_record, merit_value, incense_value);

    Ok(())

}


#[derive(Accounts)]
pub struct CreateDonateRecord<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"donate_count", authority.key().as_ref()],
        bump
     )]
     pub donate_count: Account<'info, DonateCounter>,

    #[account(
        init,
        payer = authority,
        space = 8 + DonateRecord::INIT_SPACE,
        seeds = [b"donate_record", authority.key().as_ref(),(donate_count.count+1).to_string().as_bytes()],
        bump
    )]
    pub donate_record: Account<'info, DonateRecord>,

     
    #[account(
        mut,
        seeds = [b"temple"],
        bump
    )]
    pub temple: Account<'info, Temple>,


   #[account(
    mut,
    seeds = [b"user_info",authority.key().as_ref()],
    bump
     )]
   pub user_info: Account<'info, UserInfo>,


   /// CHECK: Create a unique and indivisible NFT
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(),feats_nft_mint_account.key().as_ref(),  b"edition".as_ref(),],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub master_editon_account:UncheckedAccount<'info>,

    ///CHECK:
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(),feats_nft_mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    #[account(
        mut,
        seeds = [b"create_feats_nft",authority.key().as_ref()],
        bump,
    )]
    pub feats_nft_mint_account: Account<'info, Mint>,

    // Receive NFT account
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = feats_nft_mint_account,
        associated_token::authority = user_info,
    )]
    pub user_receive_feats_nft_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub token_metadata_program: Program<'info, Metadata>,

    pub rent: Sysvar<'info, Rent>,   

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[error_code]
pub enum DonateError {
    #[msg("Donate amount must be greater than 0")]
    InvalidDonateAmount,
    #[msg("Insufficient lamports")]
    InsufficientLamports,
}