use anchor_lang::prelude::*;

use crate::state::*;

pub fn init_incense_type(ctx: Context<InitIncenseType>, params: InitializeIncenseTypeParams) -> Result<()> {
    let incense_type_config = &mut ctx.accounts.incense_type_config;
    let temple_state = &mut ctx.accounts.temple_state;
    let current_timestamp = Clock::get()?.unix_timestamp;
    
    // 验证寺庙管理员权限
    require!(
        temple_state.is_authority(ctx.accounts.authority.key()),
        TempleError::UnauthorizedTempleAccess
    );
    
    // 初始化香型配置
    incense_type_config.initialize(params.clone(), current_timestamp)?;
    
    // 更新寺庙状态中的香型计数
    temple_state.increment_incense_type_count()?;

    // 发射事件
    emit!(IncenseInitEvent {
        incense_type_config: incense_type_config.key(),
        incense_type_id: params.incense_type_id,
        name: params.name.clone(),
        price_per_unit: params.price_per_unit,
        karma_reward: params.karma_reward,
        incense_value: params.incense_value,
        is_active: params.is_active,
        timestamp: current_timestamp,
    });
    
    // 记录成功日志
    msg!("Incense type initialized successfully");
    msg!("Incense Type ID: {}", params.incense_type_id);
    msg!("Name: {}", params.name);
    msg!("Price: {} lamports", params.price_per_unit);
    msg!("Karma Reward: {}", params.karma_reward);
    msg!("Incense Value: {}", params.incense_value);
    msg!("Is Active: {}", params.is_active);
    msg!("Total Incense Types: {}", temple_state.incense_type_count);
    
    Ok(())
}



/// 香型初始化事件
#[event]
pub struct IncenseInitEvent {
    pub incense_type_config: Pubkey,
    pub incense_type_id: u8,
    pub name: String,
    pub price_per_unit: u64,
    pub karma_reward: u32,
    pub incense_value: u32,
    pub is_active: bool,
    pub timestamp: i64,
}


#[derive(Accounts)]
#[instruction(params: InitializeIncenseTypeParams)]
pub struct InitIncenseType<'info> {

/// 香型配置账户
    /// 使用香型ID作为种子生成PDA
    #[account(
        init,                                                    // 初始化新账户
        payer = authority,                                       // 费用支付者
        space = 8 + IncenseTypeConfig::INIT_SPACE,              // 使用 InitSpace 计算空间
        seeds = [
            IncenseTypeConfig::SEED_PREFIX.as_bytes(),           
            &[params.incense_type_id],                          // 香型ID作为种子
        ],
        bump                                                     // 自动寻找有效bump
    )]
    pub incense_type_config: Account<'info, IncenseTypeConfig>,

    /// 寺庙全局状态账户
    /// 需要更新香型计数
    #[account(
        mut,                                                     // 可变，需要更新计数
        seeds = [TempleState::SEED_PREFIX.as_bytes()],           
        bump                                                     // PDA验证
    )]
    pub temple_state: Account<'info, TempleState>,

    /// 寺庙管理员账户
    /// 只有管理员可以创建香型
    #[account(mut)]  // 可变，需要支付费用
    pub authority: Signer<'info>,

    /// Solana系统程序
    pub system_program: Program<'info, System>,
    
}