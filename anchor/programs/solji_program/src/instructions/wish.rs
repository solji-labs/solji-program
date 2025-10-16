use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{
        mpl_token_metadata::types::DataV2, update_metadata_accounts_v2, Metadata,
        UpdateMetadataAccountsV2,
    },
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

use crate::{
    events::{LikeCreatedEvent, TowerNftEvent, UserActivityEvent, WishCreatedEvent},
    global_error::GlobalError,
    states::{
        create_nft, hit, ActivityEnum, NftAccounts, PublishWish, Temple, Tower, UserInfo, WishLike,
        WishUser,
    },
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

#[derive(Accounts)]
pub struct MintTowerNft<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user_info",authority.key().as_ref()],
        bump
      )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        mut,
        seeds = [b"create_tower_token",authority.key().as_ref()],
        bump,
     )]
    pub tower_nft_mint_account: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = tower_nft_mint_account,
        associated_token::authority = authority,
      )]
    pub tower_nft_associated_token_account: Account<'info, TokenAccount>,
    ///CHECK:
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(),tower_nft_mint_account.key().as_ref()],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub metadata_account: UncheckedAccount<'info>,

    /// CHECK: Create a unique and indivisible NFT
    #[account(
        mut,
        seeds = [b"metadata",token_metadata_program.key().as_ref(),tower_nft_mint_account.key().as_ref(),  b"edition".as_ref(),],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub master_editon_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
pub fn mint_tower_nft(ctx: Context<MintTowerNft>) -> Result<()> {
    // Wish Energy Tower NFT
    let user_info = &mut ctx.accounts.user_info;
    let previous_level = user_info.tower_level;
    let with_count = user_info.wish_count;
    let tower = Tower::get_tower(&with_count);
    let new_level = tower.get_level();
    msg!("tower level up: {} -> {}", previous_level, new_level);

    let aut = ctx.accounts.authority.key();
    let seeds: &[&[&[u8]]] = &[&[
        b"create_tower_token",
        aut.as_ref(),
        &[ctx.bumps.tower_nft_mint_account],
    ]];
    let ts = Clock::get()?.unix_timestamp;
    if previous_level == -1 {
        user_info.tower_level = new_level;
        let accounts = NftAccounts {
            token_metadata_program: ctx.accounts.token_metadata_program.to_account_info(),
            metadata_account: ctx.accounts.metadata_account.to_account_info(),
            nft_mint_account: ctx.accounts.tower_nft_mint_account.to_account_info(),
            payer: ctx.accounts.authority.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
            nft_associated_token_account: Some(
                ctx.accounts
                    .tower_nft_associated_token_account
                    .to_account_info(),
            ),
            master_edition_account: Some(ctx.accounts.master_editon_account.to_account_info()),
        };
        let args = tower.get_nft_args();
        create_nft(&accounts, args, seeds)?;
        msg!(
            "Tower NFT minting account info - Mint account: {}, Associated Token Account :{}",
            ctx.accounts.tower_nft_mint_account.key(),
            ctx.accounts.tower_nft_associated_token_account.key()
        );
        emit!(TowerNftEvent {
            user: ctx.accounts.authority.key(),
            mint: ctx.accounts.tower_nft_mint_account.key(),
            ata: ctx.accounts.tower_nft_associated_token_account.key(),
            previous_level,
            new_level,
            timestamp: ts,
        });
    } else if previous_level < new_level {
        user_info.tower_level = tower.get_level();
        let args = tower.get_nft_args();
        let data = DataV2 {
            name: args.name,
            symbol: args.symbol,
            uri: args.url,
            seller_fee_basis_points: 0,
            creators: None,
            collection: None,
            uses: None,
        };

        let cpi_ctx = CpiContext::new_with_signer(
            ctx.accounts.token_metadata_program.to_account_info(),
            UpdateMetadataAccountsV2 {
                metadata: ctx.accounts.metadata_account.to_account_info(),
                update_authority: ctx.accounts.tower_nft_mint_account.to_account_info(),
            },
            seeds,
        );
        update_metadata_accounts_v2(cpi_ctx, None, Some(data), Some(true), Some(true))?;
        emit!(TowerNftEvent {
            user: ctx.accounts.authority.key(),
            mint: ctx.accounts.tower_nft_mint_account.key(),
            ata: ctx.accounts.tower_nft_associated_token_account.key(),
            previous_level,
            new_level,
            timestamp: ts,
        });
    }
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
