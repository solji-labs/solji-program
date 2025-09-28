use crate::error::ErrorCode;
use crate::state::user_state::UserState;
use anchor_lang::prelude::*;

// 用户御守收藏信息
#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UserAmuletsInfo {
    pub user: Pubkey,
    // 御守数量统计
    pub total_amulets: u32,
    // 各来源御守数量
    pub draw_fortune_count: u32, // 抽签获得的数量
    pub make_wish_count: u32,    // 许愿获得的数量
    // 可铸造御守余额
    pub pending_amulets: u32, // 可铸造的御守数量
    // 最后更新时间
    pub last_updated: i64,
}

#[derive(Accounts)]
pub struct GetUserAmulets<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub user_state: Account<'info, UserState>,
}

pub fn get_user_amulets(ctx: Context<GetUserAmulets>) -> Result<UserAmuletsInfo> {
    let user = *ctx.accounts.user.key;

    // TODO 直接从用户状态获取御守信息
    Ok(UserAmuletsInfo {
        user,
        total_amulets: 0,      // 暂时设为0，后续可以从其他地方统计
        draw_fortune_count: 0, // 暂时设为0，后续可以从其他地方统计
        make_wish_count: 0,    // 暂时设为0，后续可以从其他地方统计
        pending_amulets: ctx.accounts.user_state.pending_amulets,
        last_updated: Clock::get()?.unix_timestamp,
    })
}
