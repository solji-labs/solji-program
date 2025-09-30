#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use crate::state::leaderboard::LeaderboardPeriod;
use crate::state::shop_item::ShopItem;
use crate::state::temple_config::{
    DonationLevelConfig, DonationRewardConfig, FortuneConfig, IncenseType, TempleLevelConfig,
};
use instructions::*;
use state::*;

declare_id!("D9immZaczS2ASFqqSux2iCCAaFat7vcusB1PQ2SW6d95");

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

    /// Create temple config initialization
    pub fn create_temple_config(
        ctx: Context<CreateTempleConfig>,
        treasury: Pubkey,
        regular_fortune: FortuneConfig,
        buddha_fortune: FortuneConfig,
        donation_levels: Vec<DonationLevelConfig>,
        donation_rewards: Vec<DonationRewardConfig>,
        temple_levels: Vec<TempleLevelConfig>,
    ) -> Result<()> {
        instructions::create_temple_config(
            ctx,
            treasury,
            regular_fortune,
            buddha_fortune,
            donation_levels,
            donation_rewards,
            temple_levels,
        )
    }

    /// Create NFT mint
    pub fn create_nft_mint(ctx: Context<CreateNftMint>, incense_id: u8) -> Result<()> {
        instructions::create_nft_mint(ctx, incense_id)
    }

    // /// Initialize donation leaderboard temporarily unused
    // pub fn init_donation_leaderboard(
    //     ctx: Context<InitDonationLeaderboard>,
    //     donation_deadline: u64,
    // ) -> Result<()> {
    //     instructions::init_donation_leaderboard(ctx, donation_deadline)
    // }

    /// Distribute Buddha NFT to top 10,000 donors
    // pub fn distribute_buddha_nfts(ctx: Context<DistributeBuddhaNfts>) -> Result<()> {
    //     instructions::distribute_buddha_nfts(ctx)
    // }

    /// Buy incense
    pub fn buy_incense(ctx: Context<BuyIncense>, incense_id: u8, amount: u64) -> Result<()> {
        instructions::buy_incense::buy_incense(ctx, incense_id, amount)
    }
    /// Burn incense
    pub fn burn_incense(ctx: Context<BurnIncense>, incense_id: u8, amount: u64) -> Result<()> {
        instructions::burn_incense::burn_incense(ctx, incense_id, amount)
    }

    /// Initialize user state
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        instructions::init_user(ctx)
    }

    /// Draw fortune
    pub fn draw_fortune(ctx: Context<DrawFortune>, use_merit: bool) -> Result<DrawResult> {
        instructions::draw_fortune(ctx, use_merit)
    }

    /// Share fortune to get rewards
    pub fn share_fortune(ctx: Context<ShareFortune>, share_hash: [u8; 32]) -> Result<()> {
        instructions::share_fortune(ctx, share_hash)
    }
    /// Make wish
    pub fn create_wish(
        ctx: Context<CreateWish>,
        wish_id: u64,
        content_hash: [u8; 32],
        is_anonymous: bool,
    ) -> Result<()> {
        instructions::create_wish(ctx, wish_id, content_hash, is_anonymous)
    }

    /// Like wish
    pub fn like_wish(ctx: Context<LikeWish>, wish_id: u64) -> Result<()> {
        instructions::like_wish(ctx)
    }

    /// Mint Buddha NFT
    pub fn mint_buddha_nft(ctx: Context<MintBuddhaNFT>) -> Result<()> {
        instructions::mint_buddha_nft(ctx)
    }

    /// ===== Donation instructions =====
    /// Donate fund (core donation logic)
    pub fn donate_fund(ctx: Context<DonateFund>, amount: u64) -> Result<()> {
        instructions::donate_fund(ctx, amount)
    }

    /// Process donation rewards
    pub fn process_donation_rewards(ctx: Context<ProcessDonationRewards>) -> Result<()> {
        instructions::process_donation_rewards(ctx)
    }

    /// Mint temple medal NFT
    pub fn mint_medal_nft(ctx: Context<MintMedalNFT>) -> Result<()> {
        instructions::mint_medal_nft(ctx)
    }

    /// Claim free Buddha NFT (limited to top 10000)
    // pub fn claim_buddha_nft(ctx: Context<ClaimBuddhaNft>) -> Result<()> {
    //     instructions::claim_buddha_nft(ctx)
    // }

    // /// Donate (complete flow, deprecated, keep for reference)
    // pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
    //     instructions::donation::donate(ctx, amount)
    // }

    /// Stake medal NFT
    pub fn stake_medal_nft(ctx: Context<StakeMedalNFT>) -> Result<()> {
        instructions::stake_medal_nft(ctx)
    }

    /// Unstake medal NFT
    pub fn unstake_medal_nft(ctx: Context<UnstakeMedalNFT>) -> Result<()> {
        instructions::unstake_medal_nft(ctx)
    }

    // ===== Core dynamic configuration management instructions =====

    /// Update incense type configuration
    pub fn update_incense_types(
        ctx: Context<UpdateDynamicConfig>,
        incense_types: Vec<IncenseType>,
    ) -> Result<()> {
        instructions::update_incense_types(ctx, incense_types)
    }

    /// Update fortune drawing configuration
    pub fn update_fortune_config(
        ctx: Context<UpdateDynamicConfig>,
        regular_fortune: FortuneConfig,
        buddha_fortune: FortuneConfig,
    ) -> Result<()> {
        instructions::update_fortune_config(ctx, regular_fortune, buddha_fortune)
    }

    /// Update donation level configuration
    pub fn update_donation_levels(
        ctx: Context<UpdateDynamicConfig>,
        donation_levels: Vec<DonationLevelConfig>,
    ) -> Result<()> {
        instructions::update_donation_levels(ctx, donation_levels)
    }

    /// Update donation reward configuration
    pub fn update_donation_rewards(
        ctx: Context<UpdateDynamicConfig>,
        donation_rewards: Vec<DonationRewardConfig>,
    ) -> Result<()> {
        instructions::update_donation_rewards(ctx, donation_rewards)
    }

    /// Update temple level configuration
    pub fn update_temple_levels(
        ctx: Context<UpdateDynamicConfig>,
        temple_levels: Vec<TempleLevelConfig>,
    ) -> Result<()> {
        instructions::update_temple_levels(ctx, temple_levels)
    }

    /// Update temple status
    pub fn update_temple_status(ctx: Context<UpdateTempleStatus>, status: u8) -> Result<()> {
        instructions::update_temple_status(ctx, status)
    }

    /// Update temple status by bit
    pub fn update_temple_status_by_bit(
        ctx: Context<UpdateTempleStatus>,
        bit: u8,
        disabled: bool,
    ) -> Result<()> {
        instructions::update_temple_status_by_bit(ctx, bit, disabled)
    }

    /// ====== Leaderboard =========
    /// Initialize leaderboard
    pub fn init_incense_leaderboard(ctx: Context<InitIncenseLeaderboard>) -> Result<()> {
        instructions::init_incense_leaderboard(ctx)
    }

    /// Update leaderboard
    pub fn update_leaderboard(
        ctx: Context<UpdateLeaderboard>,
        period: LeaderboardPeriod,
    ) -> Result<()> {
        instructions::update_leaderboard(ctx, period)
    }

    /// Get user ranking
    pub fn get_incense_leaderboard(
        ctx: Context<GetIncenseLeaderboard>,
    ) -> Result<IncenseLeaderBoard> {
        instructions::get_incense_leaderboard(ctx)
    }

    /// ===== Temple statistics related ======

    /// Get temple statistics data
    pub fn get_temple_stats(ctx: Context<GetTempleStats>) -> Result<TempleStats> {
        instructions::get_temple_stats(ctx)
    }

    /// Get temple level information
    pub fn get_temple_level(ctx: Context<GetTempleLevel>) -> Result<TempleLevelInfo> {
        instructions::get_temple_level(ctx)
    }

    /// === Shop related ====
    /// Get shop items list
    pub fn get_shop_items(ctx: Context<GetShopItems>) -> Result<ShopItemsResult> {
        instructions::get_shop_items(ctx)
    }

    /// Purchase shop items
    pub fn purchase_item(ctx: Context<PurchaseItem>, item_id: u8, quantity: u64) -> Result<()> {
        instructions::purchase_item(ctx, item_id, quantity)
    }

    /// Create shop configuration
    pub fn create_shop_config(
        ctx: Context<CreateShopConfig>,
        shop_items: Vec<ShopItem>,
    ) -> Result<()> {
        instructions::create_shop_config(ctx, shop_items)
    }

    /// Update shop items configuration
    pub fn update_shop_items(
        ctx: Context<UpdateShopItems>,
        shop_items: Vec<ShopItem>,
    ) -> Result<()> {
        instructions::update_shop_items(ctx, shop_items)
    }

    /// ==== User panel related ====

    /// Get user overview information
    pub fn get_user_profile(ctx: Context<GetUserProfile>) -> Result<UserProfile> {
        instructions::get_user_profile(ctx)
    }

    /// ==== User amulet related ====
    /// Get user amulet collection information
    pub fn get_user_amulets(ctx: Context<GetUserAmulets>) -> Result<UserAmuletsInfo> {
        instructions::get_user_amulets(ctx)
    }

    /// Mint amulet NFT
    pub fn mint_amulet_nft(ctx: Context<MintAmuletNFT>, source: u8) -> Result<()> {
        instructions::mint_amulet_nft(ctx, source)
    }
}
