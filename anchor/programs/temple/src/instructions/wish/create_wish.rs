use anchor_lang::prelude::*;

use crate::state::{TempleConfig, UserError, UserState, Wish};

/// 创建许愿
///
/// # Arguments
/// * `wish_id` - 许愿ID，必须等于用户当前总许愿数+1
/// * `content_hash` - 许愿内容的哈希值
/// * `is_anonymous` - 是否匿名许愿
pub fn create_wish(
    ctx: Context<CreateWish>,
    wish_id: u64,
    content_hash: [u8; 32],
    is_anonymous: bool,
) -> Result<CreateWishResult> {
    let wish = &mut ctx.accounts.wish;
    let user_state = &mut ctx.accounts.user_state;
    let user = &ctx.accounts.user;
    let temple_config = &mut ctx.accounts.temple_config;

    // 检查并重置每日限制
    user_state.check_and_reset_daily_limits();

    // 判断是否为免费许愿（当前每日许愿次数是否小于免费限制）
    let is_free_wish = user_state.get_daily_wish_count() < UserState::DAILY_FREE_WISH_LIMIT;

    // 计算需要扣除的功德值
    let reduce_karma_points = if is_free_wish {
        0
    } else {
        Wish::KARMA_COST_PER_WISH
    };

    // 判断功德值是否足够
    require!(
        user_state.get_karma_points() >= reduce_karma_points,
        UserError::NotEnoughKarmaPoints
    );

    let current_timestamp = Clock::get()?.unix_timestamp;
    // 御守概率掉落逻辑：10%概率
    let random_seed = (current_timestamp as u64).wrapping_add(wish_id);
    let amulet_drop_random = (random_seed % 100) as u8;
    let is_amulet_dropped = amulet_drop_random < 10;

    if is_amulet_dropped {
        msg!("恭喜！许愿时获得了1个御守铸造机会！ ",);
    }

    // 初始化愿望
    wish.initialize(
        wish_id,
        user.key(),
        content_hash,
        is_amulet_dropped,
        is_anonymous,
        current_timestamp,
        is_free_wish,

    )?;

    let reward_karma_points = 1u64;

    // 更新用户状态：扣除功德值，增加许愿计数
    user_state.create_wish(reduce_karma_points, reward_karma_points, current_timestamp)?;

    // 更新寺庙全局状态：增加总许愿次数
    temple_config.create_wish()?;

    let create_wish_result = CreateWishResult {
        wish_id,
        content_hash,
        is_anonymous, 
        is_free_wish,
        is_amulet_dropped, 
        current_timestamp,
        reduce_karma_points,
        reward_karma_points,
    };

    msg!("create_wish_result: {:?}", create_wish_result);

    Ok(create_wish_result)
}

#[derive(Accounts)]
#[instruction(wish_id: u64)]
pub struct CreateWish<'info> {
    /// 愿望账户 - 使用 PDA 确保唯一性
    #[account(
        init,
        payer = user,
        space = 8 + Wish::INIT_SPACE,
        seeds = [
            Wish::SEED_PREFIX.as_bytes(),
            user.key().as_ref(),
            &wish_id.to_le_bytes()
        ],
        bump,
    )]
    pub wish: Account<'info, Wish>,

    /// 用户状态账户 - 存储用户的许愿记录和功德值
    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_state: Account<'info, UserState>,

    /// 用户账户 - 交易签名者和费用支付者
    #[account(mut)]
    pub user: Signer<'info>,

    /// 寺庙全局状态账户 - 存储全局统计信息
    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Account<'info, TempleConfig>,

    /// 系统程序 - 用于创建账户
    pub system_program: Program<'info, System>,
}

#[derive(Debug, AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq)]
pub struct CreateWishResult {
    /// 许愿ID
    pub wish_id: u64,
    /// 内容哈希
    pub content_hash: [u8; 32],
    /// 是否匿名
    pub is_anonymous: bool,
    /// 是否为免费许愿
    pub is_free_wish: bool,
    /// 是否掉落御守
    pub is_amulet_dropped: bool,
    /// 奖励功德值
    pub reward_karma_points: u64,
    /// 消耗的功德值
    pub reduce_karma_points: u64, 
    /// 当前时间戳
    pub current_timestamp: i64,
}
