use crate::error::ErrorCode;
use crate::state::wish::Wish;
use anchor_lang::prelude::*;

pub fn like_wish(ctx: Context<LikeWish>) -> Result<()> {
    let wish = &mut ctx.accounts.wish_account;
    wish.likes += 1;
    Ok(())
}

#[derive(Accounts)]
pub struct LikeWish<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub wish_account: Box<Account<'info, Wish>>,
}
