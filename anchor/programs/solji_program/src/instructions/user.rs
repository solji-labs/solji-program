use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{
    events::{StakeEvent, UnstakeEvent},
    global_error::GlobalError,
    states::{Temple, UserInfo, UserStake},
};

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + UserInfo::INIT_SPACE,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        init_if_needed,
        payer = authority,
        seeds = [b"create_tower_token",authority.key().as_ref()],
        bump,
        mint::decimals = 0,
        mint::authority = tower_nft_mint_account,
        mint::freeze_authority = tower_nft_mint_account,
     )]
    pub tower_nft_mint_account: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
    let user_info = UserInfo::new(ctx.accounts.authority.key());
    ctx.accounts.user_info.set_inner(user_info);
    Ok(())
}

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        init,
        payer = authority,
        space = 8 + UserStake::INIT_SPACE,
        seeds =[b"user_stake",feats_nft_mint_account.key().as_ref(),(user_info.stake_count + 1).to_string().as_bytes()],
        bump,
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(
        mut,
        seeds = [b"create_feats_nft",authority.key().as_ref()],
        bump,
    )]
    pub feats_nft_mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = feats_nft_mint_account,
        associated_token::authority = authority,
    )]
    pub user_receive_feats_nft_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump,
    )]
    pub temple: Account<'info, Temple>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = feats_nft_mint_account,
        associated_token::authority = temple,
      )]
    pub temple_stake_associated_token_account: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}
pub fn stake(ctx: Context<Stake>) -> Result<()> {
    require!(
        ctx.accounts.user_receive_feats_nft_ata.amount > 0,
        GlobalError::NoNFTToStake
    );
    require!(
        ctx.accounts.user_receive_feats_nft_ata.owner == ctx.accounts.authority.key(),
        GlobalError::NotNFTOwner
    );

    let time = Clock::get()?.unix_timestamp;
    let stake = UserStake::new(
        ctx.accounts.authority.key(),
        ctx.accounts.feats_nft_mint_account.key(),
        time,
    );
    ctx.accounts.user_stake.set_inner(stake);

    ctx.accounts.user_info.stake_increment()?;

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_receive_feats_nft_ata.to_account_info(),
                to: ctx
                    .accounts
                    .temple_stake_associated_token_account
                    .to_account_info(),
                authority: ctx.accounts.authority.to_account_info(),
            },
        ),
        1,
    )?;

    msg!(
        "Stake Successful ata:{}",
        ctx.accounts.temple_stake_associated_token_account.key()
    );

    emit!(StakeEvent {
        user: ctx.accounts.authority.key(),
        mint: ctx.accounts.feats_nft_mint_account.key(),
        stake_account: ctx.accounts.user_stake.key(),
        timestamp: time,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeRequest<'info> {
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
        seeds = [b"create_feats_nft",authority.key().as_ref()],
        bump,
    )]
    pub feats_nft_mint_account: Account<'info, Mint>,

    #[account(mut)]
    pub user_stake: Account<'info, UserStake>,

    pub system_program: Program<'info, System>,
}
pub fn unstake_request(ctx: Context<UnstakeRequest>) -> Result<()> {
    require!(
        ctx.accounts.user_stake.status == 0,
        GlobalError::UnstakeRequestStatusError
    );

    let time = Clock::get()?.unix_timestamp;
    let day = ctx.accounts.user_stake.days_since_start_clamped(time)?;
    msg!("day:{}", day);
    if day < 7 {
        return err!(GlobalError::CantUnstakeYet);
    }

    ctx.accounts.user_stake.set_request_unstake(time)?;
    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeConfirm<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub user_stake: Account<'info, UserStake>,

    #[account(
        mut,
        seeds = [b"create_feats_nft",authority.key().as_ref()],
        bump,
    )]
    pub feats_nft_mint_account: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = feats_nft_mint_account,
        associated_token::authority = authority,
    )]
    pub user_receive_feats_nft_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump,
    )]
    pub temple: Account<'info, Temple>,

    #[account(
        mut,
        associated_token::mint = feats_nft_mint_account,
        associated_token::authority = temple,
      )]
    pub temple_stake_associated_token_account: Account<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn unstake_confirm(ctx: Context<UnstakeConfirm>) -> Result<()> {
    require!(
        ctx.accounts.user_stake.status == 1,
        GlobalError::UnstackConfirmStatusError
    );

    let time = Clock::get()?.unix_timestamp;
    ctx.accounts.user_stake.check_request_unstake(time)?;

    let user_stake = &mut ctx.accounts.user_stake;
    let day = user_stake.days_since_start_clamped(user_stake.requst_unstake_time)?;
    msg!("Unstake Confirm Day:{}", day);

    let user_info = &mut ctx.accounts.user_info;
    let reward = user_info.get_calculate_reward(day)?;
    msg!("Unstake Confirm Reward:{}", reward);

    user_info.increment_stake_merit(reward)?;
    ctx.accounts.temple.increment_stake_merit(reward)?;
    ctx.accounts.user_stake.set_unstake_confirm(time)?;

    let signer_seeds: &[&[&[u8]]] = &[&[b"temple", &[ctx.bumps.temple]]];
    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx
                    .accounts
                    .temple_stake_associated_token_account
                    .to_account_info(),
                to: ctx.accounts.user_receive_feats_nft_ata.to_account_info(),
                authority: ctx.accounts.temple.to_account_info(),
            },
        )
        .with_signer(signer_seeds),
        1,
    )?;
    emit!(UnstakeEvent {
        user: ctx.accounts.authority.key(),
        mint: ctx.accounts.feats_nft_mint_account.key(),
        stake_account: ctx.accounts.user_stake.key(),
        days_staked: day,
        reward: reward,
        timestamp: time,
    });
    Ok(())
}
