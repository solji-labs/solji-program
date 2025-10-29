use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Clock;

use crate::state::{TempleConfig, UserError, UserState};

// Switchboard 随机数预言机的最大 slot 差值（防止使用过期随机数）
const MAX_SLOT_DIFF: u64 = 10;

/// 抽签指令处理函数
/// 
/// # 参数
/// * `ctx` - 指令上下文，包含所有必需的账户
/// 
/// # 返回
/// * `Result<DrawFortuneResult>` - 抽签结果或错误
/// 
/// # 功能说明
/// 1. 检查是否为首次抽签（首次免费）
/// 2. 验证功德值是否足够（非首次需要5功德值）
/// 3. 生成随机运势结果
/// 4. 更新用户状态和寺庙统计
/// 5. 返回抽签结果
pub fn draw_fortune(ctx: Context<DrawFortune>) -> Result<DrawFortuneResult> {
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp;
    // 判断是否为首次抽签（每日首次抽签免费）
    let is_free_draw = ctx.accounts.user_state.get_daily_draw_count() == 0;

    // 计算需要消耗的功德值：首次免费，后续每次5点
    let reduce_karma_points = if is_free_draw { 0 } else { 5 };

    // 检查功德值是否足够
    require!(
        ctx.accounts.user_state.get_karma_points() >= reduce_karma_points,
        UserError::NotEnoughKarmaPoints
    );

    // 生成随机运势结果（在可变借用之前完成，避免借用冲突）
    let fortune = generate_fortune_result(&ctx, clock.unix_timestamp)?;

    // 每次抽签奖励2点功德值（鼓励用户参与）
    let reward_karma_points = 2u64;

    // 更新用户状态：扣除功德值、增加奖励、记录抽签时间
    ctx.accounts.user_state.draw_fortune(
        reduce_karma_points,
        reward_karma_points,
        current_timestamp,
    )?;

    // 更新寺庙全局统计：增加总抽签次数
    ctx.accounts.temple_config.draw_fortune()?;

    // 构建返回结果
    let draw_fortune_result = DrawFortuneResult {
        fortune,
        reduce_karma_points,
        reward_karma_points,
        current_timestamp,
        is_free_draw,
    };

    // 记录日志用于调试和监控
    msg!("抽签结果: {:?}", draw_fortune_result);

    Ok(draw_fortune_result)
}

/// 抽签结果数据结构
/// 
/// 包含抽签的所有相关信息，用于返回给客户端
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DrawFortuneResult {
    /// 本次抽签消耗的功德值
    pub reduce_karma_points: u64,
    /// 本次抽签奖励的功德值
    pub reward_karma_points: u64,
    /// 抽签时间戳
    pub current_timestamp: i64,
    /// 是否为免费抽签（每日首次）
    pub is_free_draw: bool,
    /// 运势结果
    pub fortune: FortuneResult,
}

/// 生成运势结果的辅助函数
/// 
/// # 参数
/// * `ctx` - 指令上下文，包含所有账户信息
/// * `timestamp` - 当前时间戳，用于本地环境的伪随机数生成
/// 
/// # 返回
/// * `Result<FortuneResult>` - 运势结果或错误
/// 
/// # 实现说明
/// - localnet: 使用时间戳和用户地址生成伪随机数（仅用于测试）
/// - 其他环境: 使用 Switchboard 随机数预言机提供的链上随机数
fn generate_fortune_result(ctx: &Context<DrawFortune>, timestamp: i64) -> Result<FortuneResult> {
    #[cfg(feature = "localnet")]
    {
        // 本地测试环境：使用时间戳和用户地址生成伪随机数
        // 注意：这种方式不安全，仅用于本地开发测试
        let user_key = ctx.accounts.user.key();
        let seed = timestamp
            .wrapping_add(user_key.to_bytes()[0] as i64)
            .wrapping_add(user_key.to_bytes()[31] as i64);

        let random_value = (seed.abs() % 100) as u8;
        Ok(fortune_from_random(random_value))
    }

    #[cfg(not(feature = "localnet"))]
    {
        // 生产环境：使用 Switchboard 随机数预言机
        if let Some(randomness_account) = &ctx.accounts.randomness_account {
            let clock = Clock::get()?;
            let randomness_data = randomness_account.try_borrow_data()?;
            
            // 验证随机数账户数据长度（至少需要16字节用于多个随机数）
            require!(
                randomness_data.len() >= 16,
                UserError::InvalidRandomnessAccount
            );

            // 从随机数账户读取数据并转换为 u64
            // 使用 map_err 替代 unwrap 以提供更好的错误处理
            let random_bytes: [u8; 8] = randomness_data[0..8]
                .try_into()
                .map_err(|_| UserError::InvalidRandomnessAccount)?;
            let random_u64 = u64::from_le_bytes(random_bytes);
            
            // 将随机数映射到 0-99 范围
            let random_value = (random_u64 % 100) as u8;

            msg!("使用链上随机数: {}", random_value);
            Ok(fortune_from_random(random_value))
        } else {
            // 降级方案：如果没有提供随机数账户，使用伪随机数
            // 这种情况应该在生产环境中避免
            msg!("警告：未提供随机数账户，使用降级方案");
            let clock = Clock::get()?;
            let seed = clock.unix_timestamp as u64 + clock.slot;
            let random_value = (seed % 100) as u8;
            Ok(fortune_from_random(random_value))
        }
    }
}

/// 根据随机数值映射到运势结果
/// 
/// # 参数
/// * `value` - 0-99 之间的随机数
/// 
/// # 返回
/// * `FortuneResult` - 对应的运势结果
/// 
/// # 概率分布
/// - 大吉 (GreatLuck): 5% (0-4)
/// - 吉 (Lucky): 10% (5-14)
/// - 小吉 (Good): 20% (15-34)
/// - 正常 (Normal): 30% (35-64)
/// - 小凶 (Nobad): 20% (65-84)
/// - 凶 (Bad): 10% (85-94)
/// - 大凶 (VeryBad): 5% (95-99)
fn fortune_from_random(value: u8) -> FortuneResult {
    match value {
        0..=4 => FortuneResult::GreatLuck,   // 5%: 0-4
        5..=14 => FortuneResult::Lucky,      // 10%: 5-14
        15..=34 => FortuneResult::Good,      // 20%: 15-34
        35..=64 => FortuneResult::Normal,    // 30%: 35-64
        65..=84 => FortuneResult::Nobad,     // 20%: 65-84
        85..=94 => FortuneResult::Bad,       // 10%: 85-94
        95..=99 => FortuneResult::VeryBad,   // 5%: 95-99
        _ => FortuneResult::Normal,          // 兜底（理论上不会到达）
    }
}

/// 抽签指令所需的账户结构
#[derive(Accounts)]
pub struct DrawFortune<'info> {
    /// 用户状态账户（PDA）
    /// 存储用户的功德值、抽签次数等信息
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    /// 用户账户（签名者）
    /// 必须是交易的签名者，用于身份验证
    #[account(mut)]
    pub user: Signer<'info>,

    /// 寺庙全局配置账户（PDA）
    /// 存储寺庙的全局统计信息
    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Account<'info, TempleConfig>,

    /// CHECK: Switchboard 随机数预言机账户（仅在非本地环境需要）
    /// 
    /// 在生产环境中，此账户应该是 Switchboard 随机数账户。
    /// 使用 Option 类型允许在某些测试场景下不提供此账户。
    /// 
    /// 安全性说明：
    /// - 应验证账户所有者是 Switchboard 程序
    /// - 应检查随机数的时效性（slot 差值）
    /// - 如果未提供，将降级到伪随机数（不推荐用于生产）
    #[cfg(not(feature = "localnet"))]
    pub randomness_account: Option<AccountInfo<'info>>,

    /// Solana 系统程序
    pub system_program: Program<'info, System>,
}

/// 运势结果枚举
/// 
/// 定义了7种可能的运势结果，每种结果有不同的出现概率
#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub enum FortuneResult {
    GreatLuck, // 大吉 - 5% 概率
    Lucky,     // 吉 - 10% 概率
    Good,      // 小吉 - 20% 概率
    Normal,    // 正常 - 30% 概率
    Nobad,     // 小凶 - 20% 概率
    Bad,       // 凶 - 10% 概率
    VeryBad,   // 大凶 - 5% 概率
}

impl FortuneResult {
    /// 获取运势结果的中文名称
    /// 
    /// # 返回
    /// * `&'static str` - 运势的中文名称
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

    /// 获取运势结果的详细描述
    /// 
    /// # 返回
    /// * `&'static str` - 运势的详细描述文字
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
