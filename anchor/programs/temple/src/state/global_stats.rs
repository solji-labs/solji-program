use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalStats {
    pub temple_config: Pubkey, // 关联的寺庙配置
    // 核心统计数据
    pub total_incense_points: u64,     // 总香火值
    pub total_merit: u64,              // 总功德值
    pub total_draw_fortune: u64,       // 总抽签次数
    pub total_wishes: u64,             // 总许愿次数
    pub total_donations_lamports: u64, // 总捐助金额
    pub total_users: u64,              // 总用户数
    // NFT统计
    pub total_fortune_nfts: u64,  // 总签文NFT数量
    pub total_amulets: u64,       // 总御守数量
    pub total_buddha_lights: u64, // 总佛灯数量
    // 元数据
    pub updated_at: i64, // 最后更新时间
}

impl GlobalStats {
    pub const SEED_PREFIX: &str = "global_stats_v1";

    // 转换
    pub fn total_donations_sol(&self) -> f64 {
        self.total_donations_lamports as f64 / 1_000_000_000.0
    }

    // 抽签
    pub fn increment_draw_fortune(&mut self) {
        self.total_draw_fortune = self.total_draw_fortune.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // 许愿
    pub fn increment_wishes(&mut self) {
        self.total_wishes = self.total_wishes.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // 捐助
    pub fn add_donation(&mut self, amount_lamports: u64) {
        self.total_donations_lamports = self
            .total_donations_lamports
            .saturating_add(amount_lamports);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // 签文NFT
    pub fn increment_fortune_nfts(&mut self) {
        self.total_fortune_nfts = self.total_fortune_nfts.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // TODO 护符
    pub fn increment_amulets(&mut self) {
        self.total_amulets = self.total_amulets.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // 香火值和功德值
    pub fn add_incense_value_and_merit(&mut self, incense_value: u64, merit: u64) {
        self.total_incense_points = self.total_incense_points.saturating_add(incense_value);
        self.total_merit = self.total_merit.saturating_add(merit);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // 用户数量
    pub fn increment_users(&mut self) {
        self.total_users = self.total_users.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }

    // TODO 佛灯是什么时候的
    pub fn increment_buddha_lights(&mut self) {
        self.total_buddha_lights = self.total_buddha_lights.saturating_add(1);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
    }
}
