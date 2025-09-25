#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use crate::state::leaderboard::LeaderboardPeriod;
use crate::state::shop_item::ShopItem;
use crate::state::temple_config::{
    DonationLevelConfig, DonationRewardConfig, FortuneConfig, IncenseType, TempleConfig,
    TempleLevelConfig,
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

    /// 创建寺庙配置 初始化
    pub fn create_temple_config(
        ctx: Context<CreateTempleConfig>,
        treasury: Pubkey,
        regular_fortune: FortuneConfig,
        buddha_fortune: FortuneConfig,
        donation_levels: Vec<DonationLevelConfig>,
        donation_rewards: Vec<DonationRewardConfig>,
        temple_levels: Vec<TempleLevelConfig>,
        shop_items: Vec<ShopItem>,
    ) -> Result<()> {
        instructions::create_temple_config(
            ctx,
            treasury,
            regular_fortune,
            buddha_fortune,
            donation_levels,
            donation_rewards,
            temple_levels,
            shop_items,
        )
    }

    /// 创建NFT mint
    pub fn create_nft_mint(ctx: Context<CreateNftMint>, incense_id: u8) -> Result<()> {
        instructions::create_nft_mint(ctx, incense_id)
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

    /// 初始化商城配置账户
    pub fn init_shop_config(
        ctx: Context<InitShopConfig>,
        shop_items: Vec<ShopItem>,
        incense_points_rate: u64,
        merit_rate: u64,
    ) -> Result<()> {
        instructions::init_shop_config(ctx, shop_items, incense_points_rate, merit_rate)
    }

    /// 初始化抽签配置账户
    pub fn init_fortune_config(
        ctx: Context<InitFortuneConfig>,
        fortune_config: FortuneConfig,
        buddha_fortune_config: FortuneConfig,
    ) -> Result<()> {
        instructions::init_fortune_config(ctx, fortune_config, buddha_fortune_config)
    }

    /// 初始化奖励配置账户
    pub fn init_reward_config(
        ctx: Context<InitRewardConfig>,
        donation_levels: Vec<DonationLevelConfig>,
        donation_rewards: Vec<DonationRewardConfig>,
        temple_levels: Vec<TempleLevelConfig>,
    ) -> Result<()> {
        instructions::init_reward_config(ctx, donation_levels, donation_rewards, temple_levels)
    }

    /// 更新商城物品配置
    pub fn update_shop_items(
        ctx: Context<UpdateShopConfig>,
        shop_items: Vec<ShopItem>,
    ) -> Result<()> {
        instructions::update_shop_items(ctx, shop_items)
    }

    /// 更新抽签签文配置
    pub fn update_fortune_config(
        ctx: Context<UpdateFortuneConfig>,
        regular_fortune: FortuneConfig,
        buddha_fortune: FortuneConfig,
    ) -> Result<()> {
        instructions::update_fortune_config(ctx, regular_fortune, buddha_fortune)
    }

    /// 更新捐助等级配置
    pub fn update_donation_levels(
        ctx: Context<UpdateRewardConfig>,
        donation_levels: Vec<DonationLevelConfig>,
    ) -> Result<()> {
        instructions::update_donation_levels(ctx, donation_levels)
    }

    /// 更新捐助奖励配置
    pub fn update_donation_rewards(
        ctx: Context<UpdateRewardConfig>,
        donation_rewards: Vec<DonationRewardConfig>,
    ) -> Result<()> {
        instructions::update_donation_rewards(ctx, donation_rewards)
    }

    /// 更新寺庙等级配置
    pub fn update_temple_levels(
        ctx: Context<UpdateRewardConfig>,
        temple_levels: Vec<TempleLevelConfig>,
    ) -> Result<()> {
        instructions::update_temple_levels(ctx, temple_levels)
    }

    /// 更新寺庙状态
    pub fn update_temple_status(ctx: Context<UpdateTempleStatus>, status: u8) -> Result<()> {
        instructions::update_temple_status(ctx, status)
    }

    /// 按位更新寺庙状态
    pub fn update_temple_status_by_bit(
        ctx: Context<UpdateTempleStatus>,
        bit: u8,
        disabled: bool,
    ) -> Result<()> {
        instructions::update_temple_status_by_bit(ctx, bit, disabled)
    }

    /// ====== 排行榜 =========
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

    /// ===== 寺庙统计相关 ======

    /// 获取寺庙统计数据
    pub fn get_temple_stats(ctx: Context<GetTempleStats>) -> Result<TempleStats> {
        instructions::get_temple_stats(ctx)
    }

    /// 获取寺庙等级信息
    pub fn get_temple_level(ctx: Context<GetTempleLevel>) -> Result<TempleLevelInfo> {
        instructions::get_temple_level(ctx)
    }

    /// === 商城相关 ====
    /// 获取商城物品列表
    pub fn get_shop_items(ctx: Context<GetShopItems>) -> Result<ShopItemsResult> {
        instructions::get_shop_items(ctx)
    }

    /// 购买商城物品
    pub fn purchase_item(ctx: Context<PurchaseItem>, item_id: u8, quantity: u64) -> Result<()> {
        instructions::purchase_item(ctx, item_id, quantity)
    }

    /// ==== 用户面板相关 ====

    /// 获取用户概览信息
    pub fn get_user_profile(ctx: Context<GetUserProfile>) -> Result<UserProfile> {
        instructions::get_user_profile(ctx)
    }

    /// ==== 用户御守相关 ====
    /// 获取用户御守收藏信息
    pub fn get_user_amulets(ctx: Context<GetUserAmulets>) -> Result<UserAmuletsInfo> {
        instructions::get_user_amulets(ctx)
    }

    /// Mint御守NFT
    pub fn mint_amulet_nft(ctx: Context<MintAmuletNFT>, source: u8) -> Result<()> {
        instructions::mint_amulet_nft(ctx, source)
    }
}
