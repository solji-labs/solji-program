use anchor_lang::{
    prelude::*,
    solana_program::{program::invoke, system_instruction::transfer},
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, mint_to, Burn, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    events::{DestroyEvent, IncenseBoughtEvent, IncenseBurnedEvent, UserActivityEvent},
    global_error::GlobalError,
    states::{hit, ActivityEnum, IncenseRulesConfig, IncenseType, Temple, UserInfo},
};

pub fn incense_buy(ctx: Context<IncenseBuy>, incense: u8, number: u64) -> Result<()> {
    let incense_type =
        IncenseType::get_incense_type(incense).ok_or(GlobalError::InvalidIncenseType)?;

    if incense_type == IncenseType::SecretBrewIncense
        || incense_type == IncenseType::CelestialIncense
    {
        return err!(BurnCode::InvalidIncenseType);
    }

    require!(number > 0, BurnCode::InvalidNumber);

    let amount = ctx
        .accounts
        .incense_rules_config
        .get_rule(incense_type)
        .incense_price;
    require!(amount > 0, BurnCode::InvalidAmount);

    let total_amount = number.checked_mul(amount).ok_or(BurnCode::InvalidAmount)?;

    ctx.accounts
        .user_info
        .update_incense_buy_count(incense_type, number)?;

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

    emit!(UserActivityEvent {
        user: ctx.accounts.authority.key(),
        activity_type: ActivityEnum::Burn,
        content: IncenseType::get_incense_type_to_string(incense),
        timestamp: Clock::get()?.unix_timestamp,
    });

    msg!("Transfer success total amount: {}", total_amount);
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

pub fn incense_burn(ctx: Context<CreateIncense>, incense: u8, amulet: u8) -> Result<()> {
    let incense_type =
        IncenseType::get_incense_type(incense).ok_or(GlobalError::InvalidIncenseType)?;

    if incense_type == IncenseType::SupremeSpiritIncense {
        require!(amulet == 3, GlobalError::InvalidArgs)
    }

    {
        let user_info = &mut ctx.accounts.user_info;
        check_daily_reset_and_limit(user_info, incense_type)?;
    }

    let incense_rule = {
        let incense_rules_config = &ctx.accounts.incense_rules_config;
        incense_rules_config.get_rule(incense_type)
    };

    {
        let user_info = &mut ctx.accounts.user_info;
        user_info.update_user_info(ctx.accounts.authority.key(), incense_type, incense_rule)?;
    }

    {
        let temple = &mut ctx.accounts.temple;
        temple.add_temple_incense_and_merit_attribute_upgrade(
            incense_rule.incense_value,
            incense_rule.merit_value,
        )?;
    }

    {
        let index = incense_type as usize;
        let user_info = &mut ctx.accounts.user_info;
        if !user_info.has_burn_token[index] {
            msg!("Mint burn_nft incense_type:{:?}", incense_type);
            user_info.has_burn_token[index] = true;
            let signer_seeds: &[&[&[u8]]] = &[&[
                b"create_burn_token",
                &[incense],
                &[ctx.bumps.burn_nft_mint_account],
            ]];
            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.burn_nft_mint_account.to_account_info(),
                        to: ctx
                            .accounts
                            .burn_nft_associated_token_account
                            .to_account_info(),
                        authority: ctx.accounts.burn_nft_mint_account.to_account_info(),
                    },
                    signer_seeds,
                ),
                1,
            )?;
            msg!(
                "Mint burn_nft success ata:{}",
                ctx.accounts.burn_nft_associated_token_account.key()
            );
        }
    }
    {
        let clock = Clock::get()?;
        let slot_le = clock.slot.to_le_bytes();
        let total_burn_count = ctx.accounts.temple.total_burn_count;
        let t = ctx.accounts.temple.key();
        let seeds = &[
            ctx.accounts.authority.key.as_ref(),
            t.as_ref(),
            &total_burn_count.to_le_bytes(),
            &slot_le,
        ];

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"create_amulet_token",
            &[amulet],
            &[ctx.bumps.amulet_nft_mint_account],
        ]];

        let chance = match incense_type {
            IncenseType::ClearIncense => 500,          // 5%
            IncenseType::SupremeSpiritIncense => 1000, // 10%
            _ => 0,
        };

        if chance > 0 && hit(chance, seeds) {
            ctx.accounts.user_info.amulet_increment()?;
            ctx.accounts.temple.amulet_increment()?;
            mint_to(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    MintTo {
                        mint: ctx.accounts.amulet_nft_mint_account.to_account_info(),
                        to: ctx
                            .accounts
                            .amulet_nft_associated_token_account
                            .to_account_info(),
                        authority: ctx.accounts.amulet_nft_mint_account.to_account_info(),
                    },
                    signer_seeds,
                ),
                1,
            )?;
            msg!(
                "burn mint amulet_nft success ata:{}",
                ctx.accounts.amulet_nft_associated_token_account.key()
            )
        }
    }

    emit!(IncenseBurnedEvent {
        user: ctx.accounts.authority.key(),
        incense_type,
        nft_mint: ctx.accounts.burn_nft_mint_account.key(),
        incense_value: incense_rule.incense_value,
        merit_value: incense_rule.merit_value,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

pub fn destroy(ctx: Context<Destroy>, _incense: u8) -> Result<()> {
    let m = ctx.accounts.authority.key();
    let signer_seeds: &[&[&[u8]]] = &[&[b"user_info", m.as_ref(), &[ctx.bumps.user_info]]];
    burn(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Burn {
                mint: ctx.accounts.burn_nft_mint_account.to_account_info(),
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
            signer_seeds,
        ),
        1,
    )?;

    emit!(DestroyEvent {
        user: ctx.accounts.authority.key(),
        mint: ctx.accounts.burn_nft_mint_account.key(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

pub fn check_daily_reset_and_limit(
    user_info: &mut Account<UserInfo>,
    incense_type: IncenseType,
) -> Result<()> {
    let now_ts = Clock::get()?.unix_timestamp;
    let current_day = now_ts / 86_400;
    let last_day = user_info.incense_time / 86_400;

    if current_day > last_day {
        user_info.burn_count = [0; 6];
        user_info.incense_donate_count = [0; 6];
        user_info.incense_time = now_ts;
    }

    if user_info.incense_buy_count[incense_type as usize] < 1 {
        return err!(BurnCode::BurnNotBuy);
    }

    if user_info.get_burn_count(incense_type) >= 10
        && user_info.incense_donate_count[incense_type as usize] < 1
    {
        return err!(BurnCode::TooManyBurns);
    }
    Ok(())
}

#[derive(Accounts)]
#[instruction(incense: u8,amulet: u8,)]
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
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
      )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut,
        seeds = [b"create_burn_token",&[incense]],
        bump,
       )]
    pub burn_nft_mint_account: Account<'info, Mint>,

    // Receive NFT account
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = burn_nft_mint_account,
        associated_token::authority = authority,
      )]
    pub burn_nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump,
    )]
    pub temple: Account<'info, Temple>,

    #[account(
        mut,
        seeds = [b"create_amulet_token",&[amulet]],
        bump,
     )]
    pub amulet_nft_mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = amulet_nft_mint_account,
        associated_token::authority = authority,
      )]
    pub amulet_nft_associated_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(incense: u8,)]
pub struct Destroy<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"create_burn_token",&[incense]],
        bump,
       )]
    pub burn_nft_mint_account: Account<'info, Mint>,

    #[account(
      mut,
      seeds = [b"user_info",authority.key().as_ref()],
      bump
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut,
        associated_token::mint = burn_nft_mint_account,
        associated_token::authority = authority,
      )]
    pub nft_associated_token_account: Account<'info, TokenAccount>,

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
    #[msg("Cannot burn this type of incense: you haven't purchased it yet")]
    BurnNotBuy,
}
