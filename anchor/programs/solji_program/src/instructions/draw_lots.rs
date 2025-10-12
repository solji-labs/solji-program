use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::Metadata,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use switchboard_on_demand::accounts::RandomnessAccountData;

use crate::{
    events::{CoinFlipEvent, DrawLotsEvent, UserActivityEvent},
    global_error::GlobalError,
    states::{
        hit, ActivityEnum, LotteryConfig, LotteryRecord, LotteryType, PlayerState, Temple, UserInfo,
    },
};
pub fn initialize_lottery_poetry(ctx: Context<InitializeLotteryPoetry>) -> Result<()> {
    let config = LotteryConfig::new();
    ctx.accounts.lottery_array.set_inner(config);

    let player_state = &mut ctx.accounts.player_state;
    player_state.latest_flip_result = false;
    player_state.randomness_account = Pubkey::default(); // Placeholder, will be set in coin_flip
    player_state.settled = true;
    player_state.bump = ctx.bumps.player_state;
    player_state.allowed_user = ctx.accounts.authority.key();
    player_state.commit_slot = 0;
    msg!("Initialization successful");
    Ok(())
}

#[cfg(feature = "devnet")]
use switchboard_on_demand::program_id::ON_DEMAND_DEVNET_PID as SB_OD_PID;
#[cfg(feature = "mainnet")]
use switchboard_on_demand::program_id::ON_DEMAND_MAINNET_PID as SB_OD_PID;

pub fn coin_flip(ctx: Context<CoinFlip>) -> Result<()> {
    #[cfg(any(feature = "devnet", feature = "mainnet"))]
    require_keys_eq!(
        ctx.accounts.randomness_account_data.owner,
        SB_OD_PID,
        DrawLotsCode::InvalidRandomnessAccount
    );

    let clock = Clock::get()?;
    let randomness_data =
        RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow())
            .map_err(|_| DrawLotsCode::InvalidRandomnessAccount)?;

    let d = clock.slot.saturating_sub(randomness_data.seed_slot);
    require!(d <= 3, DrawLotsCode::RandomnessExpired);

    msg!(
        "seed_slot: {},clock.slot:{}",
        randomness_data.seed_slot,
        clock.slot
    );

    let state = &mut ctx.accounts.player_state;
    require!(state.settled, DrawLotsCode::AlreadySettled);
    state.randomness_account = ctx.accounts.randomness_account_data.key();
    state.commit_slot = randomness_data.seed_slot;
    state.settled = false;

    emit!(CoinFlipEvent {
        player: ctx.accounts.authority.key(),
        randomness_account: ctx.accounts.randomness_account_data.key(),
        commit_slot: randomness_data.seed_slot,
        timestamp: clock.unix_timestamp,
    });
    Ok(())
}

pub fn draw_lots(ctx: Context<DrawLots>, amulet: u8) -> Result<()> {
    // {
    //     let st = &ctx.accounts.player_state;
    //     require_keys_eq!(
    //         st.allowed_user,
    //         ctx.accounts.authority.key(),
    //         DrawLotsCode::Unauthorized
    //     );
    //     require!(!st.settled, DrawLotsCode::AlreadySettled);
    // }

    // Fortune Omikuji
    require!(amulet == 1, GlobalError::InvalidAmulet);

    let clock = Clock::get()?;
    let now_ts = clock.unix_timestamp;
    {
        let user_info = &mut ctx.accounts.user_info;
        check_is_free(user_info, now_ts)?;
    }

    {
        let user_info = &mut ctx.accounts.user_info;
        if !user_info.lottery_is_free {
            msg!(
                "The value of the deduction money function: {}",
                user_info.merit_value
            );
            if user_info.merit_value < LotteryRecord::LOTTERY_FEE_MERIT {
                return err!(DrawLotsCode::Insufficient);
            }
            user_info.merit_value = user_info
                .merit_value
                .checked_sub(LotteryRecord::LOTTERY_FEE_MERIT)
                .ok_or(GlobalError::MathUnderflow)?;
            msg!("Remaining Merit: {}", user_info.merit_value)
        }
    }

    let lottery_type = {
        #[cfg(feature = "mainnet")]
        {
            let randomness_data =
                RandomnessAccountData::parse(ctx.accounts.randomness_account_data.data.borrow())
                    .map_err(|_| DrawLotsCode::InvalidRandomnessAccount)?;
            {
                let st = &ctx.accounts.player_state;
                require!(
                    randomness_data.seed_slot == st.commit_slot,
                    DrawLotsCode::RandomnessExpired
                );
            }

            let revealed_random_value = randomness_data
                .get_value(&clock)
                .map_err(|_| DrawLotsCode::RandomnessNotResolved)?;
            let r = u64::from_le_bytes(revealed_random_value[..8].try_into().unwrap());
            #[inline]
            fn unbiased_u64(x: u64, n: u64) -> u64 {
                ((x as u128 * n as u128) >> 64) as u64
            }
            let idx = unbiased_u64(r, 7);

            let lottery_type = ctx.accounts.lottery_array.get_lottery_type(idx);
            msg!("Random Numbers: {},Result:{:?}", idx, lottery_type);
            lottery_type
        }
        #[cfg(not(feature = "mainnet"))]
        {
            let seed =
                clock.unix_timestamp as u64 ^ ctx.accounts.authority.key().to_bytes()[0] as u64;
            let idx = seed % 7;

            let lottery_type = ctx.accounts.lottery_array.get_lottery_type(idx);
            msg!("Testnet Random Numbers: {}, Result:{:?}", idx, lottery_type);
            lottery_type
        }
    };

    let reward: u64 = if lottery_type == LotteryType::ExcellentLuck {
        3
    } else {
        2
    };

    {
        let user_info = &mut ctx.accounts.user_info;
        user_info.update_lottery_count(now_ts, reward)?;
    }

    {
        let merit_value = if ctx.accounts.user_info.lottery_is_free {
            0
        } else {
            LotteryRecord::LOTTERY_FEE_MERIT
        };
        let record = LotteryRecord::new(
            ctx.accounts.authority.key(),
            lottery_type,
            lottery_type.get_lottery_poety().to_string(),
            now_ts,
            merit_value,
        );
        ctx.accounts.lottery_record.set_inner(record);
    }

    {
        ctx.accounts.temple.add_temple_lottery()?;
    }

    // mint nft
    {
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"create_draw_token", &[ctx.bumps.draw_nft_mint_account]]];
        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.draw_nft_mint_account.to_account_info(),
                    to: ctx
                        .accounts
                        .draw_nft_associated_token_account
                        .to_account_info(),
                    authority: ctx.accounts.draw_nft_mint_account.to_account_info(),
                },
                signer_seeds,
            ),
            1,
        )?;

        msg!(
            "Mint Success ata: {}",
            ctx.accounts.draw_nft_associated_token_account.key()
        );
    }
    {
        let st = &mut ctx.accounts.player_state;
        st.settled = true;
    }

    // amulet nft
    {
        {
            let clock = Clock::get()?;
            let slot_le = clock.slot.to_le_bytes();
            let total_lottery_count = ctx.accounts.temple.total_lottery_count;
            let t = ctx.accounts.temple.key();
            let seeds = &[
                ctx.accounts.authority.key.as_ref(),
                t.as_ref(),
                &total_lottery_count.to_le_bytes(),
                &slot_le,
            ];

            let signer_seeds: &[&[&[u8]]] = &[&[
                b"create_amulet_token",
                &[amulet],
                &[ctx.bumps.amulet_nft_mint_account],
            ]];

            if hit(50, seeds) {
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
                    "draw mint amulet_nft success ata:{}",
                    ctx.accounts.amulet_nft_associated_token_account.key()
                )
            }
        }
    }

    emit!(DrawLotsEvent {
        user: ctx.accounts.authority.key(),
        lottery_type,
        lottery_poetry: lottery_type.get_lottery_poety().to_string(),
        merit_change: reward,
        timestamp: now_ts,
    });

    let content = format!(
        "{}-{}",
        lottery_type.get_lottery_to_string(),
        lottery_type.get_lottery_poety()
    );
    emit!(UserActivityEvent {
        user: ctx.accounts.authority.key(),
        activity_type: ActivityEnum::Lottery,
        content,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}

pub fn check_is_free(user_info: &mut UserInfo, now_ts: i64) -> Result<()> {
    let current_day = now_ts / 86_400;
    let last_day = user_info.lottery_time / 86_400;
    if current_day > last_day {
        user_info.lottery_is_free = true;
    }
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeLotteryPoetry<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = 8 + LotteryConfig::INIT_SPACE,
        seeds = [b"lottery_array"],
        bump
    )]
    pub lottery_array: Account<'info, LotteryConfig>,

    #[account(
        init,
        payer = authority,
        seeds = [b"playerState".as_ref(), authority.key().as_ref()],
        space = 8 + 100,
        bump)]
    pub player_state: Account<'info, PlayerState>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(amulet: u8)]
pub struct DrawLots<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
      mut,
      seeds = [b"lottery_array"],
      bump
    )]
    pub lottery_array: Account<'info, LotteryConfig>,

    #[account(
      init,
      payer = authority,
      space = 8 + LotteryRecord::INIT_SPACE,
      seeds = [b"lottery_record",authority.key().as_ref(),(user_info.lottery_count+1).to_string().as_bytes()], 
      bump
    )]
    pub lottery_record: Account<'info, LotteryRecord>,

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

    #[account(mut,
        seeds = [b"playerState".as_ref(), authority.key().as_ref()],
        bump = player_state.bump)]
    pub player_state: Account<'info, PlayerState>,

    /// CHECK: The account's data is validated manually within the handler.
    #[account(
        constraint = randomness_account_data.key() == player_state.randomness_account
          @ DrawLotsCode::InvalidRandomnessAccount
      )]
    pub randomness_account_data: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [b"create_draw_token"],
        bump,
     )]
    pub draw_nft_mint_account: Account<'info, Mint>,

    #[account(
       init_if_needed,
       payer = authority,
       associated_token::mint = draw_nft_mint_account,
       associated_token::authority = authority,
     )]
    pub draw_nft_associated_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"create_amulet_token",&[amulet]],
        bump,
     )]
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
pub struct CoinFlip<'info> {
    #[account(mut,
        seeds = [b"playerState".as_ref(), authority.key().as_ref()],
        bump = player_state.bump)]
    pub player_state: Account<'info, PlayerState>,

    pub authority: Signer<'info>,
    /// CHECK: The account's data is validated manually within the handler.
    pub randomness_account_data: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum DrawLotsCode {
    #[msg("insufficient merit value")]
    Insufficient,
    #[msg("AlreadySettled")]
    AlreadySettled,
    #[msg("RandomnessExpired")]
    RandomnessExpired,
    #[msg("InvalidRandomnessAccount")]
    InvalidRandomnessAccount,
    #[msg("RandomnessAlreadyRevealed")]
    RandomnessAlreadyRevealed,
    #[msg("RandomnessNotResolved")]
    RandomnessNotResolved,
    #[msg("Unauthorized")]
    Unauthorized,
}
