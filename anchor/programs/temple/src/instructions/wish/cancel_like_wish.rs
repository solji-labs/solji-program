use anchor_lang::prelude::*;

use crate::state::{Wish, WishError, WishLike};


pub fn cancel_like_wish(ctx: Context<CancelWishLike>, wish_id: u64) -> Result<()> {
    let like_account = &ctx.accounts.wish_like;
    let wish_account = &mut ctx.accounts.wish;
    let liker_account = &ctx.accounts.liker;

    // 验证点赞记录确实属于当前用户
    if like_account.liker != liker_account.key() {
        return Err(WishError::InvalidLiker.into());
    }

    // 验证点赞记录的愿望ID匹配
    if like_account.wish_id != wish_id {
        return Err(WishError::InvalidWish.into());
    }

    // 减少愿望的点赞数
    wish_account.cancel_like()?;

    msg!("Wish like cancelled successfully");
    
    Ok(())
}



#[derive(Accounts)]
#[instruction(wish_id: u64)]
pub struct CancelWishLike<'info> {
    /// 点赞账户 - 使用 close 回收账户租金
    #[account(
        mut,
        seeds = [
            WishLike::SEED_PREFIX.as_bytes(),
            liker.key().as_ref(),
            creator.key().as_ref(),
            &wish_id.to_le_bytes()
        ],
        bump,
        close = liker, // 将账户租金返还给点赞者
    )]
    pub wish_like: Account<'info, WishLike>,

    /// 愿望账户 - 需要更新点赞数
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
     
    /// CHECK: 愿望创建者账户
    #[account(
        constraint = creator.key() == wish.creator @ WishError::InvalidCreator
    )]
    pub creator: AccountInfo<'info>,

    /// 点赞者账户 - 交易签名者和租金接收者
    #[account(mut)]
    pub liker: Signer<'info>,
}