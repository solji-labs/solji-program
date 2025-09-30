
pub use anchor_lang::prelude::*;

use crate::state::TempleState;


pub fn init_temple(ctx: Context<InitTemple>) -> Result<()> {

    let temple_state = &mut ctx.accounts.temple_state;
    let authority = &ctx.accounts.authority.key();
    let current_timestamp = Clock::get().unwrap().unix_timestamp;
    
    temple_state.initialize(*authority, current_timestamp)?;
    
    emit!(TempleInitEvent {
        temple_state: temple_state.key(),
        authority: *authority,
        temple_level: temple_state.temple_level,
        timestamp: current_timestamp,
    });


        // 记录成功日志
    msg!("Temple state initialized successfully");
    msg!("Authority: {}", authority);
    msg!("Initial level: {}", temple_state.temple_level); 
    msg!("Created at: {}", temple_state.created_at);
    
    Ok(())
}



/// 初始化寺庙状态
#[derive(Accounts)]
pub struct InitTemple<'info> {


    /// 寺庙状态账户
    #[account(
        init,
        payer = authority,
        space = 8 + TempleState::INIT_SPACE,
        seeds = [
            TempleState::SEED_PREFIX.as_bytes(), 
        ],
        bump,
    )]
    pub temple_state: Account<'info, TempleState>,


    /// 管理员账户
    #[account(mut)]
    pub authority: Signer<'info>,

    /// 系统程序
    pub system_program: Program<'info, System>,
}


#[event]
pub struct TempleInitEvent {
    pub temple_state: Pubkey,
    pub authority: Pubkey,
    pub temple_level: u8,
    pub timestamp: i64,
}