use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

use crate::state::{TempleState, UserError, UserState};

pub fn draw_fortune(ctx: Context<DrawFortune>) -> Result<DrawResult> {
    let clock = Clock::get()?;
    
    // 判断是否为首次抽签-首次抽签不消耗功德值
    let free_draw = ctx.accounts.user_state.get_daily_draw_count() == 0;

    // 首次抽签不消耗功德值，否则消耗5功德值
    let karma_points_per_draw = if free_draw {
        0
    } else {
        5
    };
    
    // 检查功德值是否足够
    require!(
        ctx.accounts.user_state.get_karma_points() >= karma_points_per_draw, 
        UserError::NotEnoughKarmaPoints
    );

    // 生成随机运势结果（在可变借用之前完成）
    let fortune = generate_fortune_result(&ctx, clock.unix_timestamp)?;
    
    let draw_result = DrawResult {
        fortune,
        timestamp: clock.unix_timestamp,
        free_draw,
    };

    // 更新用户状态（可变借用）
    ctx.accounts.user_state.draw_fortune(karma_points_per_draw)?;

    // 更新寺庙状态（可变借用）
    ctx.accounts.temple_state.draw_fortune()?;

    // 发射抽签事件
    emit!(DrawFortuneEvent {
        user: ctx.accounts.user.key(),
        fortune: draw_result.fortune.clone(),
        timestamp: draw_result.timestamp,
        free_draw: draw_result.free_draw,
    });

    Ok(draw_result)
}

// 生成运势结果的辅助函数
fn generate_fortune_result(ctx: &Context<DrawFortune>, timestamp: i64) -> Result<FortuneResult> {
    #[cfg(feature = "localnet")]
    {
        // 本地测试环境：使用时间戳和用户地址生成伪随机数
        let user_key = ctx.accounts.user.key();
        let seed = timestamp
            .wrapping_add(user_key.to_bytes()[0] as i64)
            .wrapping_add(user_key.to_bytes()[31] as i64);
        
        let random_value = (seed.abs() % 100) as u8;
        Ok(fortune_from_random(random_value))
    }
    
    #[cfg(not(feature = "localnet"))]
    {
        // 生产环境：使用链上随机数账户
        let randomness_data = ctx.accounts.randomness_account.try_borrow_data()?;
        require!(randomness_data.len() >= 8, UserError::InvalidRandomnessAccount);
        
        // 从随机数账户读取数据（具体实现取决于你使用的随机数预言机）
        let random_bytes: [u8; 8] = randomness_data[0..8].try_into().unwrap();
        let random_u64 = u64::from_le_bytes(random_bytes);
        let random_value = (random_u64 % 100) as u8;
        
        Ok(fortune_from_random(random_value))
    }
}

// 根据随机数值映射到运势结果
fn fortune_from_random(value: u8) -> FortuneResult {
    match value {
        0..=4 => FortuneResult::GreatLuck,      // 5%: 0-4
        5..=14 => FortuneResult::Lucky,         // 10%: 5-14
        15..=34 => FortuneResult::Good,         // 20%: 15-34
        35..=64 => FortuneResult::Normal,       // 30%: 35-64
        65..=84 => FortuneResult::Nobad,        // 20%: 65-84
        85..=94 => FortuneResult::Bad,          // 10%: 85-94
        95..=99 => FortuneResult::VeryBad,      // 5%: 95-99
        _ => FortuneResult::Normal,             // 兜底
    }
}

#[derive(Accounts)]
pub struct DrawFortune<'info> {
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

    /// 寺庙状态账户
    #[account(
        mut,
        seeds = [TempleState::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_state: Account<'info, TempleState>,

    /// CHECK: 随机数账户（仅在非本地环境需要）
    #[cfg(not(feature = "localnet"))]
    pub randomness_account: AccountInfo<'info>, 

    pub system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DrawResult {
    pub fortune: FortuneResult,
    pub timestamp: i64,
    pub free_draw: bool,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum FortuneResult {
    GreatLuck,          // 大吉 5%
    Lucky,              // 吉 10%
    Good,               // 小吉 20%
    Normal,             // 正常 30%
    Nobad,              // 小凶 20%
    Bad,                // 凶 10%
    VeryBad,            // 大凶 5%
}

impl FortuneResult {
    pub fn as_str(&self) -> &'static str {
        match self {
            FortuneResult::GreatLuck => "大吉",
            FortuneResult::Lucky => "吉",
            FortuneResult::Good => "小吉",
            FortuneResult::Normal => "正常",
            FortuneResult::Nobad => "小凶",
            FortuneResult::Bad => "凶",
            FortuneResult::VeryBad => "大凶",
        }
    }

    pub fn get_description(&self) -> &'static str {
        match self {
            FortuneResult::GreatLuck => "万事顺意，心想事成",
            FortuneResult::Lucky => "诸事顺利，渐入佳境",
            FortuneResult::Good => "平平淡淡，稳中求进",
            FortuneResult::Normal => "平平淡淡，顺其自然",
            FortuneResult::Nobad => "小心谨慎，化险为夷",
            FortuneResult::Bad => "诸事不利，谨慎为上",
            FortuneResult::VeryBad => "凶险重重，静待时机",
        }
    }
}

#[event]
pub struct DrawFortuneEvent {
    pub user: Pubkey,
    pub fortune: FortuneResult,
    pub timestamp: i64,
    pub free_draw: bool,
}