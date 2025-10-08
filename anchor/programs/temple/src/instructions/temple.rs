
pub use anchor_lang::prelude::*;

use crate::state::TempleConfig;


pub fn init_temple(ctx: Context<InitTemple>,treasury: Pubkey) -> Result<()> {

    let temple_config = &mut ctx.accounts.temple_config;
    let authority = &ctx.accounts.authority.key();
    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    
    temple_config.initialize(*authority,treasury, current_timestamp)?;
    
    emit!(TempleInitEvent {
        temple_config: temple_config.key(),
        authority: *authority,
        temple_level: temple_config.temple_level,
        timestamp: current_timestamp,
    });


        // 记录成功日志
    msg!("Temple state initialized successfully");
    msg!("Authority: {}", authority);
    msg!("Initial level: {}", temple_config.temple_level); 
    msg!("Created at: {}", temple_config.created_at);
    
    Ok(())
}



/// 初始化寺庙状态
#[derive(Accounts)]
#[instruction(treasury: Pubkey)]
pub struct InitTemple<'info> {


    /// 寺庙状态账户
    #[account(
        init,
        payer = authority,
        space = 8 + TempleConfig::INIT_SPACE,
        seeds = [
            TempleConfig::SEED_PREFIX.as_bytes(), 
        ],
        bump,
    )]
    pub temple_config: Account<'info, TempleConfig>,


    /// 管理员账户
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}


#[event]
pub struct TempleInitEvent {
    pub temple_config: Pubkey,
    pub authority: Pubkey,
    pub temple_level: u8,
    pub timestamp: i64,
}