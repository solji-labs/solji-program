#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use crate::state::leaderboard::LeaderboardPeriod;
use crate::state::temple_config::{DonationLevelConfig, FortuneConfig, IncenseType, TempleConfig};
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

    /// 创建寺庙配置
    pub fn create_temple_config(
        ctx: Context<CreateTempleConfig>,
        treasury: Pubkey,
        incense_types: Vec<IncenseType>,
        regular_fortune: FortuneConfig,
        buddha_fortune: FortuneConfig,
        donation_levels: Vec<DonationLevelConfig>,
    ) -> Result<()> {
        instructions::create_temple_config(
            ctx,
            treasury,
            incense_types,
            regular_fortune,
            buddha_fortune,
            donation_levels,
        )
    }

    /// 创建NFT mint
    pub fn create_nft_mint(ctx: Context<CreateNftMint>, incense_id: u8) -> Result<()> {
        instructions::create_nft_mint(ctx, incense_id)
    }

    // buy incense
    pub fn buy_incense(ctx: Context<BuyIncense>, incense_id: u8, amount: u64) -> Result<()> {
        instructions::buy_incense(ctx, incense_id, amount)
    }

    /// 烧香
    pub fn burn_incense(ctx: Context<BurnIncense>, incense_id: u8, amount: u64) -> Result<()> {
        instructions::burn_incense::burn_incense(ctx, incense_id, amount)
    }

    /// 初始化用户状态
    pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
        instructions::init_user(ctx)
    }

    /// 抽签
    pub fn draw_fortune(ctx: Context<DrawFortune>, use_merit: bool) -> Result<DrawResult> {
        instructions::draw_fortune(ctx, use_merit)
    }

    /// 许愿
    pub fn create_wish(
        ctx: Context<CreateWish>,
        wish_id: u64,
        content_hash: [u8; 32],
        is_anonymous: bool,
    ) -> Result<()> {
        instructions::create_wish(ctx, wish_id, content_hash, is_anonymous)
    }

    /// 许愿点赞
    pub fn like_wish(ctx: Context<LikeWish>, wish_id: u64) -> Result<()> {
        instructions::like_wish(ctx)
    }

    /// Mint佛像NFT
    pub fn mint_buddha_nft(ctx: Context<MintBuddhaNFT>) -> Result<()> {
        instructions::mint_buddha_nft(ctx)
    }

    /// Mint寺庙勋章NFT
    pub fn mint_medal_nft(ctx: Context<MintMedalNFT>) -> Result<()> {
        instructions::mint_medal_nft(ctx)
    }

    /// 捐助
    pub fn donate(ctx: Context<Donate>, amount: u64) -> Result<()> {
        instructions::donate(ctx, amount)
    }

    /// 质押勋章NFT
    pub fn stake_medal_nft(ctx: Context<StakeMedalNFT>) -> Result<()> {
        instructions::stake_medal_nft(ctx)
    }

    /// 解质押勋章NFT
    pub fn unstake_medal_nft(ctx: Context<UnstakeMedalNFT>) -> Result<()> {
        instructions::unstake_medal_nft(ctx)
    }

    /// 分享签文获得奖励
    pub fn share_fortune(ctx: Context<ShareFortune>, share_hash: [u8; 32]) -> Result<()> {
        instructions::share_fortune(ctx, share_hash)
    }

    // ===== 核心动态配置管理指令 =====

    /// 更新烧香香型配置
    pub fn update_incense_types(
        ctx: Context<UpdateDynamicConfig>,
        incense_types: Vec<IncenseType>,
    ) -> Result<()> {
        instructions::update_incense_types(ctx, incense_types)
    }

    /// 更新抽签签文配置
    pub fn update_fortune_config(
        ctx: Context<UpdateDynamicConfig>,
        regular_fortune: FortuneConfig,
        buddha_fortune: FortuneConfig,
    ) -> Result<()> {
        instructions::update_fortune_config(ctx, regular_fortune, buddha_fortune)
    }

    /// 更新捐助等级配置
    pub fn update_donation_levels(
        ctx: Context<UpdateDynamicConfig>,
        donation_levels: Vec<DonationLevelConfig>,
    ) -> Result<()> {
        instructions::update_donation_levels(ctx, donation_levels)
    }

    //====== 排行榜 =========
    /// 初始化排行榜
    pub fn init_leaderboard(ctx: Context<InitLeaderboard>) -> Result<()> {
        instructions::init_leaderboard(ctx)
    }

    /// 更新排行榜
    pub fn update_leaderboard(
        ctx: Context<UpdateLeaderboard>,
        period: LeaderboardPeriod,
    ) -> Result<()> {
        instructions::update_leaderboard(ctx, period)
    }

    /// 获取用户排名
    pub fn get_user_rank(ctx: Context<GetUserRank>) -> Result<UserRankResult> {
        instructions::get_user_rank(ctx)
    }
}
