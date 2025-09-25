use crate::state::leaderboard::Leaderboard;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitLeaderboard<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + Leaderboard::INIT_SPACE,
        seeds = [Leaderboard::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub leaderboard: Box<Account<'info, Leaderboard>>,

    pub system_program: Program<'info, System>,
}

pub fn init_leaderboard(ctx: Context<InitLeaderboard>) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let bump = ctx.bumps.leaderboard;

    leaderboard.initialize(bump);

    msg!("排行榜初始化完成");

    Ok(())
}
