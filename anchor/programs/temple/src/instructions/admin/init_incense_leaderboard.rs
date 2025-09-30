use crate::state::leaderboard::Leaderboard;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitIncenseLeaderboard<'info> {
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

pub fn init_incense_leaderboard(ctx: Context<InitIncenseLeaderboard>) -> Result<()> {
    let leaderboard = &mut ctx.accounts.leaderboard;
    let bump = ctx.bumps.leaderboard;

    leaderboard.initialize(bump);

    msg!("Initialize the leaderboard");

    Ok(())
}
