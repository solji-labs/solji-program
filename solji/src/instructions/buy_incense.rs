use crate::error::ErrorCode;
use crate::state::temple_config::*;
use crate::state::user_state::{UserIncenseState, UserState};

use anchor_lang::prelude::*;
use anchor_lang::system_program::transfer;
use anchor_lang::system_program::Transfer;

pub fn buy_incense(ctx: Context<BuyIncense>, incense_id: u8, amount: u64) -> Result<()> {
    // 1. 校验amount>0
    if amount == 0 {
        return err!(ErrorCode::InvalidAmount);
    }

    // 2. 获取该香型的单价（从temple_config中读取)
    let fee_per_incense = ctx.accounts.temple_config.get_fee_per_incense(incense_id);
    let total_fee = fee_per_incense
        .checked_mul(amount)
        .ok_or(ErrorCode::MathOverflow)?;

    // 3. 校验用户SOL余额
    if ctx.accounts.authority.lamports() < total_fee {
        return err!(ErrorCode::InsufficientSolBalance);
    }

    // 4. 转账SOL到寺庙
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.authority.to_account_info(),
                to: ctx.accounts.temple_treasury.to_account_info(),
            },
        ),
        total_fee,
    )?;

    // 5. 给用户增加香余额
    ctx.accounts
        .user_incense_state
        .add_incense_balance(incense_id, amount);

    msg!(
        "User bought {} of incense type {} (total fee: {} SOL)",
        amount,
        incense_id,
        total_fee as f64 / 1e9
    ); // SOL=1e9 lamports
    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_id: u8)]
pub struct BuyIncense<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    /// CHECK: This account is validated through the constraint that ensures it matches the treasury in temple_config
    #[account(mut, constraint = temple_treasury.key() == temple_config.treasury @ ErrorCode::InvalidTempleTreasury)]
    pub temple_treasury: AccountInfo<'info>, // 寺庙国库

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + UserIncenseState::INIT_SPACE,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), authority.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
