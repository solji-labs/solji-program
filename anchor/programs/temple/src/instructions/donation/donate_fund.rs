use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::mpl_token_metadata::types::DataV2;
use anchor_spl::metadata::{
    create_metadata_accounts_v3, update_metadata_accounts_v2, CreateMetadataAccountsV3, Metadata,
    UpdateMetadataAccountsV2,
};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

use crate::state::{
    BadgeNFT, Donation, TempleConfig, UserIncenseState, UserState,
};
use crate::DonationError;
use crate::TempleError;

pub fn donate_fund(ctx: Context<DonateFund>, amount: u64) -> Result<DonateFundResult> {
    require!(amount > 0, DonationError::InvalidDonationAmount);

    // 检查支付金额是否足够
    let payment_amount = ctx.accounts.user.lamports();
    require!(payment_amount >= amount, DonationError::InsufficientPayment);

    let current_timestamp = Clock::get()?.unix_timestamp;

    let user_state = &mut ctx.accounts.user_state;
    if user_state.user == Pubkey::default() {
        user_state.initialize(ctx.accounts.user.key(), current_timestamp)?;
        msg!("User state initialized {}", ctx.accounts.user.key());
    }
 

    // 转账
    transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user.to_account_info(),
                to: ctx.accounts.temple_treasury.to_account_info(),
            },
        ),
        amount,
    )?;

    let old_donation_level = BadgeNFT::calculate_donation_level(user_state.total_donation_amount);

    let new_donation_level =
        BadgeNFT::calculate_donation_level(user_state.total_donation_amount + amount);

    msg!(
        "old_donation_level: {}, new_donation_level: {}",
        old_donation_level,
        new_donation_level
    );

    let nft_name = BadgeNFT::get_nft_name(new_donation_level);
    let uri = format!(
        "https://api.foxverse.co/temple/badge/{}/metadata.json",
        new_donation_level
    );

    if user_state.has_minted_badge_nft {
        if new_donation_level > old_donation_level {
            update_metadata_accounts_v2(
                CpiContext::new(
                    ctx.accounts.token_metadata_program.to_account_info(),
                    UpdateMetadataAccountsV2 {
                        metadata: ctx.accounts.meta_account.to_account_info(),
                        update_authority: ctx.accounts.user.to_account_info(),
                    },
                ),
                None,
                Some(DataV2 {
                    name: nft_name,
                    symbol: "TEMPLE".to_string(),
                    uri: uri,
                    seller_fee_basis_points: 0,
                    creators: None,
                    collection: None,
                    uses: None,
                }),
                None,
                if new_donation_level == 4 {
                    Some(true)
                } else {
                    None
                },
            )?;
        }
    } else {
        // Create metadata account
        let temple_signer_seeds: &[&[&[u8]]] = &[&[
            TempleConfig::SEED_PREFIX.as_bytes(),
            &[ctx.bumps.temple_config],
        ]];

        create_metadata_accounts_v3(
            CpiContext::new_with_signer(
                ctx.accounts.token_metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.meta_account.to_account_info(),
                    mint: ctx.accounts.nft_mint_account.to_account_info(),
                    mint_authority: ctx.accounts.temple_config.to_account_info(),
                    update_authority: ctx.accounts.user.to_account_info(),
                    payer: ctx.accounts.user.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                },
                temple_signer_seeds,
            ),
            DataV2 {
                name: nft_name,
                symbol: "TEMPLE".to_string(),
                uri: uri,
                seller_fee_basis_points: 0,
                creators: None,
                collection: None,
                uses: None,
            },
            true,
            true,
            None,
        )?;

        mint_to(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                MintTo {
                    mint: ctx.accounts.nft_mint_account.to_account_info(),
                    to: ctx
                        .accounts
                        .user_nft_associated_token_account
                        .to_account_info(),
                    authority: ctx.accounts.temple_config.to_account_info(),
                },
                temple_signer_seeds,
            ),
            1,
        )?;

        user_state.has_minted_badge_nft = true;
    }

    let (reward_karma_points, reward_incense_value) = Donation::calculate_donation(amount)?;
 
    //增加用户功德值
    user_state.donate_fund(
        amount,
        reward_karma_points,
        reward_incense_value,
        current_timestamp,
    )?;
    //增加寺庙功德值
    ctx.accounts
        .temple_config
        .donate_fund(amount, reward_incense_value, current_timestamp)?;

    // 如果捐助金额大于等于5sol，空投高级香型
    if amount >= 5_000_000_000 {
        let user_incense_state = &mut ctx.accounts.user_incense_state;
        if user_incense_state.user == Pubkey::default() {
            user_incense_state.initialize(ctx.accounts.user.key(), current_timestamp)?;
        }

        user_incense_state.airdrop_incense_by_donation(amount)?;
    }

    let donate_fund_result = DonateFundResult {
        reward_incense_value,
        reward_karma_points,
        donation_amount: amount,
        current_timestamp: current_timestamp,
    };

    msg!("donate_fund_result: {:?}", donate_fund_result);

    Ok(donate_fund_result)
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DonateFundResult {
    pub reward_incense_value: u64,
    pub reward_karma_points: u64,
    pub donation_amount: u64,
    pub current_timestamp: i64,
}

#[derive(Accounts)]
pub struct DonateFund<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// 用户状态账户
    #[account(
            init_if_needed,
            payer = user,
            space = 8+ UserState::INIT_SPACE,
            seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
            bump,
        )]
    pub user_state: Account<'info, UserState>,

    #[account(
        init_if_needed,
        payer = user,
        space = 8+ UserIncenseState::INIT_SPACE,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Account<'info, UserIncenseState>,
 

    /// CHECK: This account is validated through the constraint that ensures it matches the treasury in temple_config
    #[account(mut, constraint = temple_treasury.key() == temple_config.treasury @ TempleError::InvalidTreasury)]
    pub temple_treasury: AccountInfo<'info>, // 寺庙国库

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump
    )]
    pub temple_config: Account<'info, TempleConfig>,

    #[account(
        init_if_needed,
        payer = user,
        seeds = [
            BadgeNFT::SEED_PREFIX.as_bytes(),
            temple_config.key().as_ref(),
            user.key().as_ref()
        ],
        bump,
        mint::decimals = BadgeNFT::TOKEN_DECIMALS,
        mint::authority = temple_config.key(),
        mint::freeze_authority = temple_config.key(),
    )]
    pub nft_mint_account: Account<'info, Mint>,

    /// User's badge NFT associated account
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = nft_mint_account,
        associated_token::authority = user,
    )]
    pub user_nft_associated_token_account: Account<'info, TokenAccount>,

    /// CHECK: this is the metadata account
    #[account(
        mut,
        seeds = [
            b"metadata",
            token_metadata_program.key().as_ref(),
            nft_mint_account.key().as_ref(),
        ],
        bump,
        seeds::program = token_metadata_program.key(),
    )]
    pub meta_account: UncheckedAccount<'info>,

    // Program accounts
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}
