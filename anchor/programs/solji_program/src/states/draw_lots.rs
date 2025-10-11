use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // pda: 钱包 + 时间戳
pub struct LotteryRecord {
    // 抽签用户
    pub user: Pubkey,
    // 签文等级
    #[max_len(50)]
    pub lottery_type: LotteryType,

    #[max_len(64)]
    pub lottery_poetry: String,

    // 创建时间
    pub create_at: i64,
    // 功德值
    pub merit_value: u64,
}

impl LotteryRecord {
    pub const LOTTERY_FEE_MERIT: u64 = 5;

    pub const NAME: &str = "Omikuji NFT";
    pub const SYMBOL: &str = "Omikuji";
    pub const URL: &str = "https://solji.io/";

    pub fn new(
        user: Pubkey,
        lottery_type: LotteryType,
        lottery_poetry: String,
        create_at: i64,
        merit_value: u64,
    ) -> Self {
        Self {
            user,
            lottery_type,
            lottery_poetry,
            create_at,
            merit_value,
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq, Eq, InitSpace)]
pub enum LotteryType {
    // 大吉
    ExcellentLuck,
    //  中吉
    ModerateLuck,
    // 小吉
    SlightLuck,
    // 吉
    Favorable,
    // 末吉
    FutureLuck,
    // 凶
    SlightBadLuck,
    // 大凶
    TerribleLuck,
}

impl LotteryType {
    pub fn get_lottery_poety(&self) -> &str {
        match self {
            LotteryType::ExcellentLuck => "链上祥瑞至，财富节节高",
            LotteryType::ModerateLuck => "趋势已明朗，钱包鼓鼓囊",
            LotteryType::SlightLuck => "仓中资产涨，日日有进账",
            LotteryType::Favorable => "日日有小进，岁岁无烦忧",
            LotteryType::FutureLuck => "暂有小阻碍，坚持见曙光",
            LotteryType::SlightBadLuck => "链上多陷阱，操作需谨慎",
            LotteryType::TerribleLuck => "市场风浪急，三思为上计",
        }
    }
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
                LotteryType::ExcellentLuck,
                LotteryType::ModerateLuck,
                LotteryType::SlightLuck,
                LotteryType::Favorable,
                LotteryType::FutureLuck,
                LotteryType::SlightBadLuck,
                LotteryType::TerribleLuck,
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
