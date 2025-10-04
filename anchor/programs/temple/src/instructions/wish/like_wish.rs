use anchor_lang::prelude::*;

use crate::state::{Wish, WishError, WishLike};



pub fn like_wish(ctx: Context<LikeWish>, wish_id: u64) -> Result<()> {

    let like_account = &mut ctx.accounts.wish_like;

    if like_account.liker != Pubkey::default() {
        return Err(WishError::WishLikeAlreadyExists.into());
    }

    let wish_account = &mut ctx.accounts.wish;
    let creator_account = &mut ctx.accounts.creator;
    let liker_account = &mut ctx.accounts.liker;

    let timestamp = Clock::get().unwrap().unix_timestamp;

    like_account.initialize(wish_id, creator_account.key(), liker_account.key(), timestamp)?;
    
    wish_account.add_like()?;

    msg!("Wish liked successfully");
    
    Ok(())
}


#[derive(Accounts)]
#[instruction(wish_id: u64)]
pub struct LikeWish<'info> {


    /// 点赞账户 - 使用 PDA 确保唯一性
    #[account(
        init,
        payer = liker,
        space = 8 + WishLike::INIT_SPACE,
        seeds = [
            WishLike::SEED_PREFIX.as_bytes(),
            liker.key().as_ref(),
            creator.key().as_ref(),
            &wish_id.to_le_bytes()
        ],
        bump,
    )]
    pub wish_like: Account<'info, WishLike>,


    /// 愿望账户 - 使用 PDA 确保唯一性
    #[account( 
        mut,
        seeds = [
            Wish::SEED_PREFIX.as_bytes(),
            creator.key().as_ref(),
            &wish_id.to_le_bytes()
        ],
        bump,
    )]
    pub wish: Account<'info, Wish>,
     
    /// CHECK: creator account is not checked
    #[account(mut,
        constraint = creator.key() == wish.creator @ WishError::InvalidCreator)]
    pub creator: AccountInfo<'info>,


    /// 用户账户 - 交易签名者和费用支付者
    #[account(mut)]
    pub liker: Signer<'info>,


    pub system_program: Program<'info, System>,
}