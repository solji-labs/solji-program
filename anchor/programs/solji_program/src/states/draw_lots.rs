use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // pda: 钱包 + 时间戳
pub struct LotteryRecord {
    // 抽签用户
    pub user: Pubkey,
    // 签文
    #[max_len(50)]
    pub lottery_type: LotteryType,
    // 创建时间
    pub create_at: i64,
    // 功德值
    pub merit_value: u64,
}

impl LotteryRecord {
    pub const LOTTERY_FEE_MERIT: u64 = 5;
    pub fn new(user: Pubkey, lottery_type: LotteryType, create_at: i64, merit_value: u64) -> Self {
        Self {
            user,
            lottery_type,
            create_at,
            merit_value,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LotteryType {
    // 大吉
    GreatFortune,
    //  中吉
    MiddleFortune,
    // 小吉
    SmallFortune,
    // 吉
    Fortune,
    // 末吉
    LateFortune,
    // 凶
    Misfortune,
    // 大凶
    GreatMisfortune,
}
// 初始化签文
#[account]
#[derive(InitSpace)]
pub struct LotteryConfig {
    #[max_len(7)]
    pub lottery_array: [LotteryType; 7],
}

impl LotteryConfig {
    pub fn new() -> Self {
        Self {
            lottery_array: [
                LotteryType::GreatFortune,
                LotteryType::MiddleFortune,
                LotteryType::SmallFortune,
                LotteryType::Fortune,
                LotteryType::LateFortune,
                LotteryType::Misfortune,
                LotteryType::GreatMisfortune,
            ],
        }
    }

    pub fn get_lottery_type(&self, index: u64) -> LotteryType {
        self.lottery_array[index as usize]
    }
}

#[account]
pub struct PlayerState {
    pub allowed_user: Pubkey,       // 哪个钱包可以玩这个游戏（避免别人随便调用）
    pub latest_flip_result: bool,   // 上一次抛硬币的结果 (true=正面，false=反面)
    pub randomness_account: Pubkey, // Switchboard 的随机账户地址（存储这一轮用的随机源）
    pub commit_slot: u64,           // 承诺的 slot
    pub settled: bool,              // 是否完成
    pub bump: u8,                   // PDA bump，用于种子签名
}
