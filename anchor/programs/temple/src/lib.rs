#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod instructions;
pub mod state;

use instructions::*;
use state::*; 

declare_id!("81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o");

pub mod admin {
    use super::{pubkey, Pubkey};
    #[cfg(feature = "devnet")]
    pub const ID: Pubkey = pubkey!("DRayqG9RXYi8WHgWEmRQGrUWRWbhjYWYkCRJDd6JBBak");
    #[cfg(feature = "localnet")]
    pub const ID: Pubkey = pubkey!("FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU");
    #[cfg(not(any(feature = "devnet", feature = "localnet")))]
    pub const ID: Pubkey = pubkey!("FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU");
}

#[program]
pub mod temple {

    use super::*;
 
    pub fn init_temple(ctx: Context<InitTemple>,treasury: Pubkey) -> Result<()> {
        instructions::temple::init_temple(ctx,treasury)
    }


    pub fn init_incense_type(ctx: Context<InitIncenseType>, params: InitializeIncenseTypeParams) -> Result<()> {
        instructions::incense::init_incense_type(ctx, params)
    }

    pub fn init_incense_nft(ctx: Context<InitIncenseNft>, incense_type_id: u8) -> Result<()> {
        instructions::incense::init_incense_nft(ctx, incense_type_id)
    }

    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        instructions::user::init_user(ctx)
    }

    pub fn buy_incense<'info>(ctx: Context<'_, '_, 'info, 'info, BuyIncense<'info>>, buy_incense_params: Vec<BuyIncenseItem>) -> Result<()> {
        instructions::buy_incense::buy_incense(ctx, buy_incense_params)
    }

    pub fn burn_incense<'info>(ctx: Context< BurnIncense>,incense_type_id: u8, amount: u8) -> Result<()> {
        instructions::burn_incense::burn_incense(ctx, incense_type_id, amount)
    }

    pub fn burn_incense_simplied<'info>(ctx: Context< BurnIncenseSimplied>,incense_type_id: u8, amount: u8, payment_amount: u64) -> Result<()> {
        instructions::burn_incense_simplied::burn_incense_simplied(ctx, incense_type_id, amount, payment_amount)
    }

    pub fn draw_fortune(ctx: Context< DrawFortune>) -> Result<DrawResult> {
        instructions::fortune::draw_fortune(ctx)
    }

    pub fn create_wish(ctx: Context< CreateWish>, wish_id: u64, content_hash: [u8; 32], is_anonymous: bool) -> Result<CreateWishResult> {
        instructions::wish::create_wish(ctx, wish_id, content_hash, is_anonymous)
    }

    pub fn like_wish(ctx: Context< LikeWish>, wish_id: u64) -> Result<()> {
        instructions::wish::like_wish(ctx, wish_id)
    }

    pub fn cancel_like_wish(ctx: Context<CancelWishLike>, wish_id: u64) -> Result<()> {
        instructions::wish::cancel_like_wish(ctx, wish_id)
    }

    pub fn mint_buddha_nft(ctx: Context<MintBuddhaNft>) -> Result<()> {
        instructions::mint_buddha_nft::mint_buddha_nft(ctx)
    }

    pub fn donate_fund(ctx: Context<DonateFund>,amount: u64) -> Result<()> {
        instructions::donation::donate_fund(ctx,amount)
    }
}
