#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("EHeMFMeN22VbNmB9BnPaQLRFVVGRuw5uEU6bVSM3eWaw");

pub mod events;
pub mod global_error;
pub mod instructions;
pub mod states;

use instructions::*;
use states::*;

#[program]
pub mod sol_ji {

    use super::*;

    /// 创建寺庙
    pub fn create_temple(ctx: Context<CreateTemple>) -> Result<()> {
        instructions::create_temple(ctx)
    }

    ///  temple 提现
    pub fn withdraw(ctx: Context<Withdraw>, lamports: u64) -> Result<()> {
        instructions::withdraw(ctx, lamports)
    }

    /// 初始化香配置
    pub fn initialize(ctx: Context<InitializeIncense>) -> Result<()> {
        instructions::initialize(ctx)
    }

    /// 管理员修改规则
    pub fn update_incense(
        ctx: Context<UpdateIncense>,
        a: IncenseType,
        b: IncenseRule,
    ) -> Result<()> {
        instructions::update_incense(ctx, a, b)
    }

    /// 创建用户烧香,抽签,许愿前调用一次创建用户信息
    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        instructions::create_user(ctx)
    }

    /// 购买香
    pub fn incense_buy(ctx: Context<IncenseBuy>, args: u8, number: u64) -> Result<()> {
        instructions::incense_buy(ctx, args, number)
    }

    /// 烧香
    pub fn incense_burn(ctx: Context<CreateIncense>, incense: u8, amulet: u8) -> Result<()> {
        instructions::incense_burn(ctx, incense, amulet)
    }

    /// 销毁nft
    pub fn destroy(ctx: Context<Destroy>, _incense: u8) -> Result<()> {
        instructions::destroy(ctx, _incense)
    }

    /// 初始化签文
    pub fn initialize_lottery_poetry(ctx: Context<InitializeLotteryPoetry>) -> Result<()> {
        instructions::initialize_lottery_poetry(ctx)
    }

    /// 随机数前置指令
    pub fn coin_flip(ctx: Context<CoinFlip>) -> Result<()> {
        instructions::coin_flip(ctx)
    }

    /// 抽签
    pub fn draw_lots(ctx: Context<DrawLots>, amulet: u8) -> Result<()> {
        instructions::draw_lots(ctx, amulet)
    }

    /// 许愿
    pub fn create_wish(
        ctx: Context<CreateWish>,
        content: String,
        is_anonymous: bool,
        // amulet: u8,
    ) -> Result<()> {
        instructions::create_wish(ctx, content, is_anonymous /* amulet */)
    }

    /// 点赞
    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::create_like(ctx)
    }

    /// 创建烧香nft
    pub fn burn_incense_nft_mint(ctx: Context<CreateBurnToken>, args: u8) -> Result<()> {
        instructions::burn_incense_nft_mint(ctx, args)
    }

    /// sbt nft
    pub fn mint_sbt_nft(ctx: Context<MintSbtNft>) -> Result<()> {
        instructions::mint_sbt_nft(ctx)
    }

    pub fn draw_mint_nft(ctx: Context<DrawMintNft>) -> Result<()> {
        instructions::draw_mint_nft(ctx)
    }

    pub fn wish_mint_nft(ctx: Context<WishMintNft>) -> Result<()> {
        instructions::wish_mint_nft(ctx)
    }

    pub fn amulet_mint_nft(ctx: Context<AmuletMintNft>, amulet: u8) -> Result<()> {
        instructions::amulet_mint_nft(ctx, amulet)
    }
    /// 捐助计数器
    pub fn create_donate_count(ctx: Context<CreateDonateCount>) -> Result<()> {
        instructions::create_donate_count(ctx)
    }

    /// 捐助
    pub fn create_donate_record(ctx: Context<CreateDonateRecord>, amount: u64) -> Result<()> {
        instructions::create_donate_record(ctx, amount)
    }
}
