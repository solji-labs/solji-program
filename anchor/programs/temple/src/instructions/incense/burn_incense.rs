use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::metadata::Metadata;
use anchor_spl::metadata::{
    create_metadata_accounts_v3, mpl_token_metadata::types::DataV2, CreateMetadataAccountsV3,
};
use anchor_spl::token::{mint_to, Mint, MintTo, Token, TokenAccount};

use crate::state::BurnIncenseError;
use crate::state::IncenseNFT;
use crate::state::IncenseTypeConfig;
use crate::state::TempleConfig;
use crate::state::UserIncenseState;
use crate::state::UserState;

pub fn burn_incense(ctx: Context<BurnIncense>, incense_type_id: u8, amount: u8) -> Result<()> {
    let user_state = &mut ctx.accounts.user_state;
    let user_incense_state = &mut ctx.accounts.user_incense_state;
    let temple_config = &mut ctx.accounts.temple_config;
    let incense_type_config = &mut ctx.accounts.incense_type_config;

    // 检查香型配置是否激活
    require!(
        incense_type_config.is_active,
        BurnIncenseError::IncenseTypeNotActive
    );

    // 检查烧香数量是否合法
    require!(amount > 0 && amount <= 10, BurnIncenseError::InvalidAmount);

    // 检查用户香型余额是否足够
    require!(
        user_incense_state.get_incense_having_balance(incense_type_id) >= amount.into(),
        BurnIncenseError::NotEnoughIncense
    );

    // 检查用户今日烧香次数是否已用完
    require!(
        user_state.get_available_burn_operations() > 0,
        BurnIncenseError::DailyBurnLimitExceeded
    );

    let temple_config_key = temple_config.key();

    let signer_seeds: &[&[&[u8]]] = &[&[
        IncenseNFT::SEED_PREFIX.as_bytes(),
        temple_config_key.as_ref(),
        &[incense_type_id],
        &[ctx.bumps.nft_mint_account],
    ]];

    // Mint 一个功德香NFT给用户作为烧香的证明

    // 1. 铸造NFT token到用户关联账户
    let mint_amount = 1u64; // NFT数量为1
    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            MintTo {
                mint: ctx.accounts.nft_mint_account.to_account_info(),
                to: ctx
                    .accounts
                    .user_nft_associated_token_account
                    .to_account_info(),
                authority: ctx.accounts.nft_mint_account.to_account_info(),
            },
            signer_seeds,
        ),
        mint_amount,
    )?;
 

    // 烧香
    // into()表示将u8转换为u32
    user_state.burn_incense(
        incense_type_config.karma_reward.into(),
        incense_type_config.incense_value.into(),
        amount.into(),
    )?;

    // 消耗香
    user_incense_state.sub_incense_balance(incense_type_id, amount.into())?;

    temple_config.add_incense_value(incense_type_config.incense_value.into())?;

    Ok(())
}

#[derive(Accounts)]
#[instruction(incense_type_id: u8)]
pub struct BurnIncense<'info> {
    /// 香型配置账户
    #[account(
        mut,
        seeds = [
            IncenseTypeConfig::SEED_PREFIX.as_bytes(),
             &[incense_type_id]
             ],
        bump,
    )]
    pub incense_type_config: Account<'info, IncenseTypeConfig>,

    /// CHECK: 寺庙管理员账号
    #[account(mut,
        constraint = temple_authority.key() == temple_config.authority @ BurnIncenseError::InvalidOwner)]
    pub temple_authority: AccountInfo<'info>,

    /// 寺庙状态账户
    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Account<'info, TempleConfig>,

    /// 用户香型状态账户
    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Account<'info, UserIncenseState>,

    /// 用户状态账户
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    /// 用户账户
    #[account(mut)]
    pub user: Signer<'info>,

    /// nft mint
    #[account(
        init_if_needed,
        payer = user,
        seeds = [
            IncenseNFT::SEED_PREFIX.as_bytes(),
               temple_config.key().as_ref(),
               &[incense_type_id]],
        bump,
        mint::decimals = IncenseNFT::TOKEN_DECIMALS,
        mint::authority = nft_mint_account.key(),
        mint::freeze_authority = temple_authority.key(),
    )]
    pub nft_mint_account: Account<'info, Mint>,

    /// 用户NFT关联账户
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

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
