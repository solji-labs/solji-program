use crate::error::ErrorCode;
use crate::state::event::RewardsProcessed;
use crate::state::global_stats::GlobalStats;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserDonationState, UserIncenseState};
use anchor_lang::prelude::*;

/// 处理捐助奖励指令
/// 监听捐助完成事件，处理等级奖励和动态配置奖励
#[derive(Accounts)]
pub struct ProcessDonationRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    #[account(
        mut,
        seeds = [GlobalStats::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub global_stats: Account<'info, GlobalStats>,

    #[account(
        mut,
        seeds = [UserDonationState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_donation_state: Box<Account<'info, UserDonationState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,
}

pub fn process_donation_rewards(ctx: Context<ProcessDonationRewards>) -> Result<()> {
    let clock = Clock::get()?;

    // 获取捐助等级奖励
    let (merit_reward, incense_points_reward) =
        ctx.accounts.user_donation_state.get_donation_rewards();

    // 更新用户香火状态
    if merit_reward > 0 || incense_points_reward > 0 {
        ctx.accounts
            .user_incense_state
            .add_incense_value_and_merit(incense_points_reward, merit_reward);

        msg!(
            "捐助获得奖励 - 功德值: {}, 香火值: {}",
            merit_reward,
            incense_points_reward
        );
    }

    // 更新全局统计
    ctx.accounts
        .global_stats
        .add_incense_value_and_merit(incense_points_reward, merit_reward);

    // 处理捐助解锁香逻辑 - 从动态配置读取
    let total_donation_sol =
        ctx.accounts.user_donation_state.donation_amount as f64 / 1_000_000_000.0;

    // 遍历所有捐助奖励配置
    for reward_config in &ctx.accounts.temple_config.dynamic_config.donation_rewards {
        // 检查是否达到最低捐助金额门槛
        if total_donation_sol >= reward_config.min_donation_sol {
            // 计算当前应该获得的奖励总量
            let current_reward = if reward_config.burn_bonus_per_001_sol > 0 {
                // 烧香次数奖励：每0.01SOL增加的烧香次数
                ((total_donation_sol * 100.0) as u64)
                    .saturating_mul(reward_config.burn_bonus_per_001_sol)
            } else {
                // 香奖励：基于门槛的累积奖励
                let current_tier = (total_donation_sol / reward_config.min_donation_sol) as u64;
                current_tier.saturating_mul(reward_config.incense_amount)
            };

            // 这里可以记录已处理的奖励，避免重复发放
            // 实际实现中需要额外的状态跟踪

            if current_reward > 0 {
                if reward_config.burn_bonus_per_001_sol > 0 {
                    // 烧香次数奖励
                    ctx.accounts.user_incense_state.incense_number = ctx
                        .accounts
                        .user_incense_state
                        .incense_number
                        .saturating_add(current_reward as u8);
                    msg!("捐助获得额外烧香次数: {}", current_reward);
                } else {
                    // 香奖励
                    ctx.accounts
                        .user_incense_state
                        .add_incense_balance(reward_config.incense_id, current_reward);
                    msg!(
                        "捐助解锁香类型{}: {} 根",
                        reward_config.incense_id,
                        current_reward
                    );
                }
            }
        }
    }

    // 发出奖励处理完成事件
    emit!(RewardsProcessed {
        user: ctx.accounts.user.key(),
        merit_reward,
        incense_points_reward,
        timestamp: clock.unix_timestamp,
    });

    Ok(())
}
