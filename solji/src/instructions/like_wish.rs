use crate::error::ErrorCode;
use crate::state::wish::Wish;
use anchor_lang::prelude::*;

pub fn like_wish(ctx: Context<LikeWish>) -> Result<()> {
    let wish = &mut ctx.accounts.wish_account;
    wish.likes = wish.likes.checked_add(1).ok_or(ErrorCode::MathOverflow)?;
    Ok(())
}

#[derive(Accounts)]
#[instruction(wish_id: u64)]
pub struct LikeWish<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"wish", wish_account.creator.as_ref(), &wish_id.to_le_bytes()],
        bump = wish_account.bump,
        constraint = wish_account.creator != user.key() @ ErrorCode::CannotLikeOwnWish
    )]
    pub wish_account: Box<Account<'info, Wish>>,
}
