use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken, metadata::Metadata, token::{burn, Burn, Mint, Token, TokenAccount}
};

use crate::{events::{DestroyEvent, IncenseBoughtEvent, IncenseBurned}, states::{create_master_edition, create_metadata, mint_nft, CreateNftArgs, IncenseBurnArgs, IncenseRulesConfig, IncenseType, NftAccounts, Temple, UserInfo}};

// 香的类型 incense_type
// count 购买香的数量
pub fn incense_buy(ctx:Context<IncenseBuy>,incense_type: IncenseType,number:u64)->Result<()>{
    // 秘制香,天界香无法购买
    if incense_type == IncenseType::SecretIncense || incense_type == IncenseType::CelestialIncense {
        return err!(BurnCode::InvalidIncenseType);
    }

    require!(number > 0, BurnCode::InvalidNumber);

    let amount = ctx.accounts.incense_rules_config.get_rule(incense_type).incense_price;
    require!(amount > 0, BurnCode::InvalidAmount);

    let total_amount = number.checked_mul(amount).ok_or(BurnCode::InvalidAmount)?;

    ctx.accounts.user_info.update_incense_property_count(incense_type, number)?;

    // 转账
    let tx = transfer(
        &ctx.accounts.authority.key(),
        &ctx.accounts.temple.key(),
        total_amount,
    );

    invoke(
        &tx,
        &[
            ctx.accounts.authority.to_account_info(),
            ctx.accounts.temple.to_account_info(),
        ],
    )?;

    emit!(IncenseBoughtEvent {
        buyer: ctx.accounts.authority.key(),
        incense_type,
        number,
        unit_price: amount,
        total_amount,
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Transfer success total amount: {}",total_amount);
    Ok(())
}


#[derive(Accounts)]
pub struct IncenseBuy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
      )]
      pub user_info: Account<'info, UserInfo>,

      #[account(
        mut,
        seeds = [b"incense_rules_config"] ,
        bump
    )]
      pub incense_rules_config: Account<'info, IncenseRulesConfig>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump
    )]
    pub temple: Account<'info, Temple>,

    pub system_program: Program<'info, System>,

}

pub fn incense_burn(ctx: Context<CreateIncense> ,args: IncenseBurnArgs) -> Result<()> {
    let incense_type  = args.incense_type;

    let user_info = &mut ctx.accounts.user_info;

    if user_info.incense_property_count[incense_type as usize] < 1 {
        return err!(BurnCode::BurningIncenseFailed);
    }

    check_daily_reset_and_limit(user_info, incense_type)?;

    let name = args.name.clone();
    let seeds: &[&[&[u8]]] = &[&[
        b"create_burn_token",
        ctx.accounts.authority.key.as_ref(),
        name.as_bytes(),
        &[ctx.bumps.nft_mint_account],
    ]];

    // mint nft 
    let accounts = NftAccounts {
        token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
        metadata_account: ctx.accounts.metadata_account.to_account_info(),
        nft_mint_account: ctx.accounts.nft_mint_account.to_account_info(),
        authority: ctx.accounts.authority.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        rent: ctx.accounts.rent.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        nft_associated_token_account: ctx.accounts.user_receive_nft_ata.to_account_info(),
        master_edition_account: ctx.accounts.master_editon_account.to_account_info(),
    };
    
    create_metadata(&accounts, CreateNftArgs{
        name:args.name,
        symbol: args.symbol,
        url: args.url,
        is_mutable: args.is_mutable,
        collection_details: args.collection_details,
    }, seeds)?;

    mint_nft(&accounts,seeds)?;

    create_master_edition(&accounts,seeds)?;
    
    let incense_rule = {
        let incense_rules_config = &ctx.accounts.incense_rules_config;
        incense_rules_config.get_rule(incense_type)
    };

    // 增加用户烧香次数
    {
        user_info.update_user_info(ctx.accounts.authority.key(), incense_type, incense_rule)?;
    }

    // 增加寺庙香火值
    {
        let temple =  &mut ctx.accounts.temple;
        temple.add_temple_incense_and_merit_attribute(incense_rule.incense_value, incense_rule.merit_value)?;
    }

    emit!(IncenseBurned {
        user: ctx.accounts.authority.key(),
        incense_type,
        nft_mint: ctx.accounts.nft_mint_account.key(),
        incense_value: incense_rule.incense_value,
        merit_value: incense_rule.merit_value,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}


pub fn destroy(ctx: Context<Destroy>) -> Result<()> {
    let m = ctx.accounts.authority.key();
    let signer_seeds: &[&[&[u8]]] =
        &[&[b"user_info", m.as_ref(), &[ctx.bumps.user_info]]];
    burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                from: ctx.accounts.user_receive_nft_ata.to_account_info(),
                authority: ctx.accounts.user_info.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    emit!(DestroyEvent {
        user: ctx.accounts.authority.key(),
        mint: ctx.accounts.nft_mint_account.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}


// 提取的检查函数
pub fn check_daily_reset_and_limit(
    user_info: &mut Account<UserInfo>,
    incense_type: IncenseType,
) -> Result<()> {
    let now_ts = Clock::get()?.unix_timestamp;
    let last_day = (user_info.incense_time + 8 * 3600) / 86400;
    let current_day = (now_ts + 8 * 3600) / 86400;
    // 处理每日重置逻辑
    if current_day > last_day {
        user_info.burn_count = [0; 6];
        user_info.donate_count = 0;
        user_info.incense_time = now_ts;
    }
    // 检查是否超过最大次数
    if user_info.get_burn_count(incense_type) >= 10 && user_info.donate_count == 0 {
        return err!(BurnCode::TooManyBurns);
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(args: IncenseBurnArgs)]
pub struct CreateIncense<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [b"incense_rules_config"] ,
      bump
  )]
    pub incense_rules_config: Account<'info, IncenseRulesConfig>,

    #[account(
      init_if_needed,
      payer = authority,
      space = 8 + UserInfo::INIT_SPACE,
      seeds = [b"user_info",authority.key().as_ref()],
      bump
    )]
    pub user_info: Account<'info, UserInfo>,

     /// CHECK:创建唯一不可分割的nft
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
        init,
        payer = authority, 
        seeds = [b"create_burn_token",authority.key().as_ref(),args.name.as_bytes()],
        mint::decimals = 0,
        mint::authority = nft_mint_account,
        mint::freeze_authority = nft_mint_account,
        bump,
       )]
     pub nft_mint_account: Account<'info, Mint>,

    // 接收nft账户
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = nft_mint_account,
        associated_token::authority = user_info,
      )]
    pub user_receive_nft_ata: Account<'info, TokenAccount>,

      #[account(
        mut,
        seeds = [b"temple"],
        bump
    )]
    pub temple: Account<'info, Temple>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,

    pub token_metadata_program: Program<'info, Metadata>,
}


#[derive(Accounts)]
pub struct Destroy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub nft_mint_account: Account<'info, Mint>,

    #[account(
      mut,
      seeds = [b"user_info",authority.key().as_ref()],
      bump
    )]
    pub user_info: Account<'info, UserInfo>,

    // 接收nft账户
    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = user_info,
      )]
    pub user_receive_nft_ata: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

#[error_code]
pub enum BurnCode {

    #[msg("Purchase incense burner")]
    BurningIncenseFailed,

    #[msg("Too many burns")]
    TooManyBurns,

    #[msg("Invalid incense type")]
    InvalidIncenseType,

    #[msg("Invalid amount")]
    InvalidAmount,

    #[msg("Invalid number")]
    InvalidNumber,

    
}
