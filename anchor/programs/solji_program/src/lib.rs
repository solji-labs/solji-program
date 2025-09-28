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

    /// 废弃
    pub fn nft_mint(ctx: Context<CreateBurnToken>, args: CreateNftArgs) -> Result<()> {
        instructions::nft_mint(ctx, args)
    }

    /// 购买香
    pub fn incense_buy(
        ctx: Context<IncenseBuy>,
        incense_type: IncenseType,
        number: u64,
    ) -> Result<()> {
        instructions::incense_buy(ctx, incense_type, number)
    }

    /// 烧香
    pub fn incense_burn(ctx: Context<CreateIncense>, args: IncenseBurnArgs) -> Result<()> {
        instructions::incense_burn(ctx, args)
    }

    /// 销毁nft
    pub fn destroy(ctx: Context<Destroy>) -> Result<()> {
        instructions::destroy(ctx)
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
    pub fn draw_lots(ctx: Context<DrawLots>) -> Result<()> {
        instructions::draw_lots(ctx)
    }

    /// 许愿
    pub fn create_wish(
        ctx: Context<CreateWish>,
        content: String,
        is_anonymous: bool,
    ) -> Result<()> {
        instructions::create_wish(ctx, content, is_anonymous)
    }

    /// 点赞
    pub fn create_like(ctx: Context<CreateLike>) -> Result<()> {
        instructions::create_like(ctx)
    }

    /// sbt nft
    pub fn mint_sbt_nft(ctx: Context<MintSbtNft>, args: CreateNftArgs) -> Result<()> {
        instructions::mint_sbt_nft(ctx, args)
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
