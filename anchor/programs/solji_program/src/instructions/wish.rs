use anchor_lang::prelude::*;

use crate::{
    events::{LikeCreated, WishCreated},
    states::{PublishWish, Temple, UserInfo, WishLike},
};
// 许愿 value是扣除功德值
pub fn create_wish(
    ctx: Context<CreateWish>,
    content: String,
    value: u64,
    is_anonymous: bool,
) -> Result<()> {
    require!(value > 0, WishCode::InvalidValue);
    {
        let user_info = &mut ctx.accounts.user_info;
        user_info.check_is_free();
        // 扣除功德值
        user_info.check_wish_daily_count(value)?;
        // 许愿一次功德值+1
        user_info.update_user_wish_count()?;
    }

    {
        let content_for_publish = content.clone();
        let publish_wish = if is_anonymous {
            PublishWish::new(Pubkey::default(), content_for_publish)
        } else {
            PublishWish::new(ctx.accounts.authority.key(), content_for_publish)
        };
        ctx.accounts.publish_wish.set_inner(publish_wish);
    }

    {
        ctx.accounts.temple.add_temple_wish()?;
    }

    emit!(WishCreated {
        user: ctx.accounts.authority.key(),
        content,
        value,
        is_anonymous
    });

    Ok(())
}

pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
    let wish_key = ctx.accounts.publish_wish.key();
    let publish_wish = &mut ctx.accounts.publish_wish;
    publish_wish.like_count += 1;

    ctx.accounts
        .wish_like
        .set_inner(WishLike::new(ctx.accounts.authority.key(), wish_key));

    emit!(LikeCreated {
        liker: ctx.accounts.authority.key(),
        wish: wish_key,
        new_like_count: publish_wish.like_count,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

#[derive(Accounts)]
pub struct CreateWishUser<'info> {
    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
      )]
    pub user_info: Account<'info, UserInfo>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateWish<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      init,
      payer = authority,
      space = 8 + PublishWish::INIT_SPACE,
      seeds = [b"publish_wish",user_info.key().as_ref(),(user_info.wish_total_count+1).to_string().as_bytes()], 
      bump
    )]
    pub publish_wish: Account<'info, PublishWish>,

    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
      )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut,
        seeds = [b"temple"],
        bump
    )]
    pub temple: Account<'info, Temple>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // 许愿
    #[account(mut)]
    pub publish_wish: Account<'info, PublishWish>,

    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
      )]
    pub user_info: Account<'info, UserInfo>,

    // 点赞
    #[account(
      init,
      payer = authority,
      space = 8 + WishLike::INIT_SPACE,
      seeds = [b"wish_like",user_info.key().as_ref(),publish_wish.key().as_ref()], 
      bump
    )]
    pub wish_like: Account<'info, WishLike>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum WishCode {
    #[msg("insufficient merit value")]
    Insufficient,

    #[msg("invalid value")]
    InvalidValue,
}
