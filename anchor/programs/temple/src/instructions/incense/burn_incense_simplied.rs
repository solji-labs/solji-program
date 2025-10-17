use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::state::{BurnIncenseError, IncenseNFT, IncenseTypeConfig, TempleConfig, UserState};
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

pub fn burn_incense_simplied(
    ctx: Context<BurnIncenseSimplied>,
    incense_type_id: u8,
    amount: u8,
    payment_amount: u64,
) -> Result<()> {
    require!(amount > 0 && amount <= 10, BurnIncenseError::InvalidAmount);
    require!(
        ctx.accounts.incense_type_config.is_active,
        BurnIncenseError::InactiveIncenseType
    );

    let total_paid = ctx.accounts.incense_type_config.price_per_unit * amount as u64;

    require!(
        payment_amount == total_paid,
        BurnIncenseError::InvalidPaymentAmount
    );

    require!(
        ctx.accounts.user.lamports() >= total_paid,
        BurnIncenseError::NotEnoughSol
    );

    let current_timestamp = Clock::get().unwrap().unix_timestamp;

    let user_state = &mut ctx.accounts.user_state;
    let temple_config = &mut ctx.accounts.temple_config;

    if user_state.user == Pubkey::default() {
        user_state.initialize(ctx.accounts.user.key(), current_timestamp)?;
    }

    user_state.check_and_reset_daily_limits()?;

    // 检查每日烧香次数是否超过限制
    require!(
        user_state.get_available_burn_operations() > 0,
        BurnIncenseError::DailyBurnLimitExceeded
    );

    // 转账
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.temple_authority.to_account_info(),
            },
        ),
        total_paid,
    )?;

    let temple_config_key = temple_config.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        IncenseNFT::SEED_PREFIX.as_bytes(),
        temple_config_key.as_ref(),
        &[incense_type_id],
        &[ctx.bumps.nft_mint_account],
    ]];

    // Mint 一个功德香NFT给用户作为烧香的证明

    // 1. 铸造NFT token到用户关联账户
    let mint_amount = 1u64; // NFT数量为1
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx
                    .accounts
                    .user_nft_associated_token_account
                    .to_account_info(),
                authority: ctx.accounts.nft_mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        mint_amount,
    )?;

    // 烧香
    user_state.burn_incense(
        ctx.accounts.incense_type_config.karma_reward.into(),
        ctx.accounts.incense_type_config.incense_value.into(),
        amount.into(),
    )?;

    temple_config.add_incense_value(ctx.accounts.incense_type_config.incense_value.into())?;

    ctx.accounts
        .incense_type_config
        .increment_minted_count(amount.into())?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_type_id: u8)]
pub struct BurnIncenseSimplied<'info> {
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
        constraint = temple_authority.key() == temple_config.authority @ BurnIncenseError::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    /// 寺庙状态账户
    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Account<'info, TempleConfig>,

    /// 用户状态账户
    #[account(
        init_if_needed,
        payer = user,
        space = 8 + UserState::INIT_SPACE,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    /// 用户账户
    #[account(mut)]
    pub user: Signer<'info>,

    /// nft mint
    #[account(
        init_if_needed,
        payer = user,
        seeds = [
            IncenseNFT::SEED_PREFIX.as_bytes(),
               temple_config.key().as_ref(),
               &[incense_type_id]],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_authority.key(),
    )]
    pub nft_mint_account: Account<'info, Mint>,

    /// 用户NFT关联账户
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint_account,
        associated_token::authority = user,
    )]
    pub user_nft_associated_token_account: Account<'info, TokenAccount>,

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

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
