use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    events::{LikeCreatedEvent, UserActivityEvent, WishCreatedEvent},
    global_error::GlobalError,
    states::{hit, ActivityEnum, PublishWish, Temple, UserInfo, WishLike, WishUser},
};
pub fn create_wish(
    ctx: Context<CreateWish>,
    content: String,
    is_anonymous: bool,
    amulet: u8,
) -> Result<()> {
    // Protection Omikuji
    require!(amulet == 2, GlobalError::InvalidAmulet);

    {
        let user_info = &mut ctx.accounts.user_info;
        user_info.check_is_free()?;
        user_info.check_wish_daily_count(WishUser::WISH_FEE as u64)?;
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

    // mint nft
    {
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"create_wish_token", &[ctx.bumps.wish_nft_mint_account]]];
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.wish_nft_mint_account.to_account_info(),
                    to: ctx
                        .accounts
                        .wish_nft_associated_token_account
                        .to_account_info(),
                    authority: ctx.accounts.wish_nft_mint_account.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        msg!(
            "Mint Success ata: {}",
            ctx.accounts.wish_nft_associated_token_account.key()
        );
    }

    {
        // amulet nft
        {
            {
                let clock = Clock::get()?;
                let slot_le = clock.slot.to_le_bytes();
                let total_wish_count = ctx.accounts.temple.total_wish_count;
                let t = ctx.accounts.temple.key();
                let seeds = &[
                    ctx.accounts.authority.key.as_ref(),
                    t.as_ref(),
                    &total_wish_count.to_le_bytes(),
                    &slot_le,
                ];

                let amulet_seeds: &[&[u8]] = &[b"create_amulet_token", &[amulet]];
                let (_pda, bump) = Pubkey::find_program_address(amulet_seeds, ctx.program_id);
                let signer_seeds: &[&[&[u8]]] = &[&[b"create_amulet_token", &[amulet], &[bump]]];

                if hit(10, seeds) {
                    ctx.accounts.user_info.amulet_increment()?;
                    ctx.accounts.temple.amulet_increment()?;
                    mint_to(
                        CpiContext::new_with_signer(
                            ctx.accounts.token_program.to_account_info(),
                            MintTo {
                                mint: ctx.accounts.amulet_nft_mint_account.to_account_info(),
                                to: ctx
                                    .accounts
                                    .amulet_nft_associated_token_account
                                    .to_account_info(),
                                authority: ctx.accounts.amulet_nft_mint_account.to_account_info(),
                            },
                            signer_seeds,
                        ),
                        1,
                    )?;
                    msg!(
                        "wish mint amulet_nft success ata:{}",
                        ctx.accounts.amulet_nft_associated_token_account.key()
                    )
                }
            }
        }
    }

    let c = content.clone();
    emit!(WishCreatedEvent {
        user: ctx.accounts.authority.key(),
        content: c,
        value: WishUser::WISH_FEE,
        is_anonymous,
        timestamp: Clock::get()?.unix_timestamp,
    });

    emit!(UserActivityEvent {
        user: ctx.accounts.authority.key(),
        activity_type: ActivityEnum::Wish,
        content: content.clone(),
        timestamp: Clock::get()?.unix_timestamp,
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

    emit!(LikeCreatedEvent {
        liker: ctx.accounts.authority.key(),
        wish: wish_key,
        new_like_count: publish_wish.like_count,
        timestamp: Clock::get()?.unix_timestamp,
    });

    emit!(UserActivityEvent {
        user: ctx.accounts.authority.key(),
        activity_type: ActivityEnum::Like,
        content: wish_key.to_string(),
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
#[instruction(amulet: u8)]
pub struct CreateWish<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      init,
      payer = authority,
      space = 8 + PublishWish::INIT_SPACE,
      seeds = [b"publish_wish",user_info.key().as_ref(),(user_info.wish_count+1).to_string().as_bytes()],
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
        bump,
    )]
    pub temple: Account<'info, Temple>,

    #[account(
        mut,
        seeds = [b"create_wish_token"],
        bump,
     )]
    pub wish_nft_mint_account: Account<'info, Mint>,

    #[account(
       init_if_needed,
       payer = authority,
       associated_token::mint = wish_nft_mint_account,
       associated_token::authority = authority,
     )]
    pub wish_nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub amulet_nft_mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = amulet_nft_mint_account,
        associated_token::authority = authority,
      )]
    pub amulet_nft_associated_token_account: Account<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct CreateLike<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub publish_wish: Account<'info, PublishWish>,

    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
      )]
    pub user_info: Account<'info, UserInfo>,

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
