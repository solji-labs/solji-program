use crate::error::ErrorCode;
use crate::state::medal_nft::*;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

#[derive(Accounts)]
pub struct StakeMedalNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), owner.key().as_ref()],
        bump = user_state.bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        mut,
        seeds = [MedalNFT::SEED_PREFIX.as_bytes(), b"account", temple_config.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub medal_nft_account: Box<Account<'info, MedalNFT>>,

    #[account(
        mut,
        seeds = [
            MedalNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            owner.key().as_ref()
        ],
        bump,
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// 用户的勋章NFT关联账户
    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = owner,
    )]
    pub nft_associated_token_account: Box<Account<'info, TokenAccount>>,

    /// 合约的勋章NFT代币账户（用于质押）
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = nft_mint_account,
        associated_token::authority = temple_config,
    )]
    pub staked_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn stake_medal_nft(ctx: Context<StakeMedalNFT>) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    let medal_nft = &mut ctx.accounts.medal_nft_account;
    let now = Clock::get()?.unix_timestamp;

    // 检查用户是否拥有勋章NFT
    require!(user_state.has_medal_nft, ErrorCode::UserDoesNotHaveMedalNFT);

    // 检查勋章是否已经被质押
    require!(medal_nft.staked_at.is_none(), ErrorCode::MedalAlreadyStaked);

    // 将NFT转移到合约账户进行质押
    token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.nft_associated_token_account.to_account_info(),
                to: ctx.accounts.staked_nft_token_account.to_account_info(),
                authority: ctx.accounts.owner.to_account_info(),
            },
        ),
        1,
    )?;

    // 更新勋章状态
    medal_nft.staked_at = Some(now);

    msg!("mint success");

    Ok(())
}

#[derive(Accounts)]
pub struct UnstakeMedalNFT<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), owner.key().as_ref()],
        bump = user_state.bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), owner.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        mut,
        seeds = [MedalNFT::SEED_PREFIX.as_bytes(), b"account", temple_config.key().as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub medal_nft_account: Box<Account<'info, MedalNFT>>,

    #[account(
        mut,
        seeds = [
            MedalNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            owner.key().as_ref()
        ],
        bump,
    )]
    pub nft_mint_account: Box<Account<'info, Mint>>,

    /// 用户的勋章NFT关联账户
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = nft_mint_account,
        associated_token::authority = owner,
    )]
    pub nft_associated_token_account: Box<Account<'info, TokenAccount>>,

    /// 合约的勋章NFT代币账户
    #[account(
        mut,
        associated_token::mint = nft_mint_account,
        associated_token::authority = temple_config,
    )]
    pub staked_nft_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn unstake_medal_nft(ctx: Context<UnstakeMedalNFT>) -> Result<()> {
    let medal_nft = &mut ctx.accounts.medal_nft_account;
    let now = Clock::get()?.unix_timestamp;

    // 检查勋章是否已质押
    let staked_at = medal_nft.staked_at.ok_or(ErrorCode::MedalNotStaked)?;

    // 计算质押时间（秒）
    let staking_duration = now - staked_at;
    let min_staking_days = 7 * 24 * 60 * 60; // 7天

    // 是否达到一周质押时间 是则奖励
    if staking_duration >= min_staking_days {
        let reward = calculate_staking_reward(staking_duration);
        ctx.accounts
            .user_incense_state
            .add_incense_value_and_merit(0, reward);
        msg!("质押成功，获得奖励: {} 功德值", reward);
    } else {
        msg!("质押时间不足7天，无法获得奖励");
    }

    // 将NFT转回用户账户
    let temple_signer_seeds: &[&[&[u8]]] = &[&[
        TempleConfig::SEED_PREFIX.as_bytes(),
        &[ctx.bumps.temple_config],
    ]];

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.staked_nft_token_account.to_account_info(),
                to: ctx.accounts.nft_associated_token_account.to_account_info(),
                authority: ctx.accounts.temple_config.to_account_info(),
            },
            temple_signer_seeds,
        ),
        1,
    )?;

    // 更新勋章状态
    medal_nft.staked_at = None;

    msg!("勋章NFT解质押完成");

    Ok(())
}

//TODO 质押奖励暂时按照每7天+30
fn calculate_staking_reward(duration_seconds: i64) -> u64 {
    let weeks = duration_seconds / (7 * 24 * 60 * 60);
    if weeks >= 1 {
        30 + (weeks as u64 - 1) * 30
    } else {
        0
    }
}
