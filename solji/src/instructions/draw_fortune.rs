use crate::error::ErrorCode;
use crate::state::temple_config::TempleConfig;
use crate::state::user_state::{UserIncenseState, UserState};
use anchor_lang::prelude::*;

// 定义签文枚举
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub enum FortuneResult {
    GreatLuck,    // 大吉
    GoodLuck,     // 中吉
    Neutral,      // 平
    BadLuck,      // 凶
    GreatBadLuck, // 大凶
}

impl FortuneResult {
    pub fn as_str(&self) -> &str {
        match self {
            FortuneResult::GreatLuck => "大吉",
            FortuneResult::GoodLuck => "中吉",
            FortuneResult::Neutral => "平",
            FortuneResult::BadLuck => "凶",
            FortuneResult::GreatBadLuck => "大凶",
        }
    }

    pub fn get_description(&self) -> &str {
        match self {
            FortuneResult::GreatLuck => "万事顺利，心想事成",
            FortuneResult::GoodLuck => "诸事顺利，渐入佳境",
            FortuneResult::Neutral => "平平淡淡，稳中求进",
            FortuneResult::BadLuck => "小心谨慎，逢凶化吉",
            FortuneResult::GreatBadLuck => "多加小心，静观其变",
        }
    }
}

// 定义抽签结果结构
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DrawResult {
    pub fortune: FortuneResult,
    pub timestamp: i64,
    pub used_merit: bool,
}

#[derive(Accounts)]
pub struct DrawFortune<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [UserState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump = user_state.bump,
    )]
    pub user_state: Box<Account<'info, UserState>>,

    #[account(
        mut,
        seeds = [UserIncenseState::SEED_PREFIX.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub user_incense_state: Box<Account<'info, UserIncenseState>>,

    #[account(
        seeds = [TempleConfig::SEED_PREFIX.as_bytes()],
        bump,
    )]
    pub temple_config: Box<Account<'info, TempleConfig>>,

    /// CHECK: 随机数账户（仅在非本地环境需要）
    #[cfg(not(feature = "localnet"))]
    pub randomness_account: AccountInfo<'info>,
}

pub fn draw_fortune(ctx: Context<DrawFortune>, use_merit: bool) -> Result<DrawResult> {
    let user_state: &mut Account<'_, UserState> = &mut ctx.accounts.user_state;
    let now = Clock::get()?.unix_timestamp;

    // 检查是否可以使用功德值抽签
    if use_merit {
        ctx.accounts.user_incense_state.consume_merit_for_draw(5)?;
    } else {
        // 检查是否可以免费抽签
        if !ctx.accounts.user_incense_state.can_draw_free() {
            return err!(ErrorCode::DailyIncenseLimitExceeded);
        }
    }

    // 生成随机数：根据编译特征决定使用哪种方式
    #[cfg(feature = "localnet")]
    let random_value = {
        // 本地测试环境：使用系统时钟作为伪随机数种子
        let clock = Clock::get()?;
        let seed = clock.unix_timestamp as u64 + clock.slot;
        (seed % 100) as u8
    };

    #[cfg(not(feature = "localnet"))]
    let random_value = {
        // 生产环境：使用Switchboard预言机随机数
        let clock = Clock::get()?;

        // 解析随机数账户数据
        let randomness_data = switchboard_on_demand::RandomnessAccountData::parse(
            ctx.accounts.randomness_account.data.borrow(),
        )
        .map_err(|_| ErrorCode::InvalidRandomness)?;

        // 获取随机数
        let revealed_random_value = randomness_data
            .get_value(clock.slot)
            .map_err(|_| ErrorCode::RandomnessNotResolved)?;

        // 从随机数数组中提取一个u64值
        let mut random_bytes = [0u8; 8];
        random_bytes.copy_from_slice(&revealed_random_value[..8]);
        let random_u64 = u64::from_le_bytes(random_bytes);

        // 转换为0-99的随机数
        (random_u64 % 100) as u8
    };

    // 从动态配置中获取概率设置
    let fortune_config = ctx
        .accounts
        .temple_config
        .get_fortune_config(user_state.has_buddha_nft);

    // 根据动态配置的概率分配签文
    if user_state.has_buddha_nft {
        msg!("佛像持有者获得概率加成");
    }

    let fortune = {
        let mut cumulative_prob = 0u8;
        cumulative_prob += fortune_config.great_luck_prob;
        if random_value < cumulative_prob {
            FortuneResult::GreatLuck
        } else {
            cumulative_prob += fortune_config.good_luck_prob;
            if random_value < cumulative_prob {
                FortuneResult::GoodLuck
            } else {
                cumulative_prob += fortune_config.neutral_prob;
                if random_value < cumulative_prob {
                    FortuneResult::Neutral
                } else {
                    cumulative_prob += fortune_config.bad_luck_prob;
                    if random_value < cumulative_prob {
                        FortuneResult::BadLuck
                    } else {
                        FortuneResult::GreatBadLuck
                    }
                }
            }
        }
    };

    // 更新用户抽签计数
    ctx.accounts.user_incense_state.update_draw_count();

    // 给予功德奖励（免费抽签+2功德）
    if !use_merit {
        ctx.accounts
            .user_incense_state
            .add_incense_value_and_merit(0, 2);
    }

    let fortune_str = fortune.as_str();
    let fortune_desc = fortune.get_description();

    msg!("抽签结果: {}", fortune_str);
    msg!("签文解释: {}", fortune_desc);

    let result = DrawResult {
        fortune,
        timestamp: now,
        used_merit: use_merit,
    };

    Ok(result)
}
