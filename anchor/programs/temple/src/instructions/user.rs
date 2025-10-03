use anchor_lang::prelude::*;

use crate::state::*;


/// 初始化用户状态
pub fn init_user(ctx: Context<InitUser>) -> Result<()> {

    let user_state = &mut ctx.accounts.user_state;
    let user = ctx.accounts.user.key();
    let current_timestamp = Clock::get()?.unix_timestamp;

    // 初始化用户状态
    user_state.initialize(user, current_timestamp)?;

    // 发射事件
    emit!(UserInitEvent {
        user_state: user_state.key(),
        user,
        timestamp: current_timestamp,
    });

        // 记录成功日志
        msg!("User state initialized successfully");
        msg!("User: {}", user);
        msg!("User State PDA: {}", ctx.accounts.user_state.key());
        msg!("Created at: {}", current_timestamp);
        msg!("Daily burn limit: {}", UserState::DAILY_BURN_LIMIT);
        msg!("Daily wish limit: {}", UserState::DAILY_WISH_LIMIT);

    Ok(())
}




/// 初始化用户所需的账户结构
#[derive(Accounts)]
pub struct InitUser<'info> {
    /// 用户状态账户
    /// 使用用户地址作为种子生成PDA
    #[account(
        init,                                           // 初始化新账户
        payer = user,                                   // 账户创建费用支付者
        space = 8 + UserState::INIT_SPACE,             // 使用InitSpace自动计算空间
        seeds = [
            UserState::SEED_PREFIX.as_bytes(),           
            user.key().as_ref(),                        // 用户地址作为种子
        ],
        bump                                            // 自动寻找有效的bump值
    )]
    pub user_state: Account<'info, UserState>,

    /// 用户账户
    /// 用户为自己创建状态账户
    #[account(mut)]  // 可变，因为需要支付账户创建费用
    pub user: Signer<'info>,

    /// Solana系统程序
    /// 用于创建新账户
    pub system_program: Program<'info, System>,
}

/// 用户初始化事件
/// 用于客户端监听和数据同步
#[event]
pub struct UserInitEvent {
    pub user_state: Pubkey,
    pub user: Pubkey,
    pub timestamp: i64,
}