use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

use crate::state::{TempleConfig, UserDonationState, UserError, UserIncenseState, UserState};
use crate::DonationError;
use crate::TempleError; 

pub fn donate_fund(ctx: Context<DonateFund>, amount: u64) -> Result<()> {
    require!(amount > 0, DonationError::InvalidDonationAmount);

    let current_timestamp = Clock::get()?.unix_timestamp;

    let user_state = &mut ctx.accounts.user_state;
    if user_state.user == Pubkey::default() {
        user_state.initialize(ctx.accounts.user.key(), current_timestamp)?;
        msg!("User state initialized {}", ctx.accounts.user.key());
    }

    let user_donation_state = &mut ctx.accounts.user_donation_state;
    if user_donation_state.user == Pubkey::default() {
        user_donation_state.initialize(ctx.accounts.user.key())?;
        msg!(
            "User donation state initialized {}",
            ctx.accounts.user.key()
        );
    }

    // 检查支付金额是否足够
    let payment_amount = ctx.accounts.user.lamports();
    require!(payment_amount >= amount, DonationError::InsufficientPayment);

    // 转账
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.temple_treasury.to_account_info(),
            },
        ),
        amount,
    )?;

    //捐助
    let _total_donation_amount = user_donation_state.donate_fund(amount, current_timestamp)?;
    //增加用户功德值
    user_state.donate_fund(amount, current_timestamp)?;
    //增加寺庙功德值
    ctx.accounts
        .temple_config
        .donate_fund(amount, current_timestamp)?;

    // 如果捐助金额大于等于5sol，空投高级香型
    if amount >= 5_000_000_000 {
        let user_incense_state = &mut ctx.accounts.user_incense_state;
        if user_incense_state.user == Pubkey::default() {
            user_incense_state.initialize(ctx.accounts.user.key(), current_timestamp)?;
        }

        user_incense_state.airdrop_incense_by_donation(amount)?;
        
    }

    Ok(())
}

#[derive(Accounts)]
pub struct DonateFund<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// 用户状态账户
    #[account(
            init_if_needed,
            payer = user,
            space = 8+ UserState::INIT_SPACE,
            seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
            bump,
        )]
    pub user_state: Account<'info, UserState>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8+ UserIncenseState::INIT_SPACE,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Account<'info, UserIncenseState>,

    /// 用户捐助状态账户
    #[account(
            init_if_needed,
            payer = user,
            space = 8+ UserDonationState::INIT_SPACE,
            seeds = [UserDonationState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
            bump,
        )]
    pub user_donation_state: Account<'info, UserDonationState>,

    /// CHECK: This account is validated through the constraint that ensures it matches the treasury in temple_config
    #[account(mut, constraint = temple_treasury.key() == temple_config.treasury @ TempleError::InvalidTreasury)]
    pub temple_treasury: AccountInfo<'info>, // 寺庙国库

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump
    )]
    pub temple_config: Account<'info, TempleConfig>,

    pub system_program: Program<'info, System>,
}
