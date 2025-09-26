use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{events::TempleWithdrawal, states::Temple};

pub fn create_temple(ctx: Context<CreateTemple>) -> Result<()> {
    let temple = Temple::new(ctx.accounts.authority.key());
    ctx.accounts.temple.set_inner(temple);
    Ok(())
}

pub fn withdraw(ctx: Context<Withdraw>, lamports: u64) -> Result<()> {
    if lamports <= 0 {
        return err!(WithdrawCode::AmountMustBeGreaterThanZero);
    }

    let rent = Rent::get()?;
    let data_len = ctx.accounts.temple.to_account_info().data_len();
    let min_rent = rent.minimum_balance(data_len);

    let total = ctx.accounts.temple.to_account_info().lamports();
    require!(
        total > min_rent,
        WithdrawCode::AmountMustBeLessThanTempleBalance
    );
    let available = total - min_rent;

    require!(
        lamports <= available,
        WithdrawCode::AmountMustBeLessThanTempleBalance
    );

    let signer_seeds: &[&[&[u8]]] = &[&[b"temple", &[ctx.bumps.temple]]];
    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.temple.to_account_info(),
                to: ctx.accounts.admin.to_account_info(),
            },
            signer_seeds,
        ),
        lamports,
    )?;

    let remaining = ctx.accounts.temple.to_account_info().lamports();
    emit!(TempleWithdrawal {
        admin: ctx.accounts.admin.key(),
        amount: lamports,
        remaining_balance: remaining,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateTemple<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + Temple::INIT_SPACE,
        seeds = [b"temple"],
        bump
    )]
    pub temple: Account<'info, Temple>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump,
        has_one = admin,
    )]
    pub temple: Account<'info, Temple>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum WithdrawCode {
    #[msg("Withdraw amount must be greater than 0")]
    AmountMustBeGreaterThanZero,

    #[msg("Withdraw amount exceeds available balance (after reserving rent-exempt minimum).")]
    AmountMustBeLessThanTempleBalance,
}
