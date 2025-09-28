use anchor_lang::prelude::*;

// 捐助排行榜用户条目
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace)]
pub struct DonationUser {
    pub user: Pubkey,
    pub total_donation: u64, // 总捐助金额（lamports）
}

// 捐助排行榜账户
#[account]
#[derive(InitSpace)]
pub struct DonationLeaderboard {
    pub bump: u8,
    pub total_donors: u32,            // 总捐助人数
    pub donation_deadline: u64,       // 捐助截止时间戳
    pub distribution_completed: bool, // 是否已完成分配
    pub distributed_count: u32,       // 已分配NFT数量
    pub last_updated: i64,            // 最后更新时间
    // 存储前10,000名捐助者（按捐助金额降序排列）
    #[max_len(10000)]
    pub top_donors: Vec<DonationUser>,
}

impl DonationLeaderboard {
    pub const SEED_PREFIX: &'static str = "donation_leaderboard";

    // 初始化排行榜
    pub fn initialize(&mut self, bump: u8, donation_deadline: u64) {
        self.bump = bump;
        self.total_donors = 0;
        self.donation_deadline = donation_deadline;
        self.distribution_completed = false;
        self.distributed_count = 0;
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    // 更新用户捐助记录
    pub fn update_donation(&mut self, user: Pubkey, donation_amount: u64) {
        let now = Clock::get().unwrap().unix_timestamp;

        // 检查是否在截止时间前
        if (now as u64) >= self.donation_deadline {
            return; // 截止后不更新排行榜
        }

        // 查找现有条目
        if let Some(existing_entry) = self.top_donors.iter_mut().find(|u| u.user == user) {
            // 更新现有用户的捐助金额
            existing_entry.total_donation = existing_entry
                .total_donation
                .saturating_add(donation_amount);
        } else {
            // 添加新用户
            self.top_donors.push(DonationUser {
                user,
                total_donation: donation_amount,
            });
            self.total_donors = self.total_donors.saturating_add(1);
        }

        // 重新排序（降序）
        self.top_donors
            .sort_by(|a, b| b.total_donation.cmp(&a.total_donation));

        // 保持前10,000名
        if self.top_donors.len() > 10000 {
            self.top_donors.truncate(10000);
        }

        self.last_updated = now;
    }

    // 获取用户排名（返回排名位置，0-based）
    pub fn get_user_rank(&self, user: &Pubkey) -> Option<u32> {
        self.top_donors
            .iter()
            .position(|u| u.user == *user)
            .map(|pos| pos as u32)
    }

    // 检查用户是否在前10,000名
    pub fn is_top_donor(&self, user: &Pubkey) -> bool {
        self.get_user_rank(user).is_some()
    }

    // 获取前N名捐助者
    pub fn get_top_donors(&self, limit: usize) -> &[DonationUser] {
        let len = self.top_donors.len().min(limit);
        &self.top_donors[..len]
    }

    // 标记分配完成
    pub fn mark_distribution_completed(&mut self, distributed_count: u32) {
        self.distribution_completed = true;
        self.distributed_count = distributed_count;
        self.last_updated = Clock::get().unwrap().unix_timestamp;
    }

    // 检查是否可以开始分配
    pub fn can_start_distribution(&self, current_time: u64) -> bool {
        current_time >= self.donation_deadline && !self.distribution_completed
    }

    // 获取待分配的用户列表（前10,000名中没有Buddha NFT的用户）
    pub fn get_eligible_donors(&self, users_with_buddha: &[Pubkey]) -> Vec<DonationUser> {
        self.top_donors
            .iter()
            .filter(|donor| !users_with_buddha.contains(&donor.user))
            .take(10000)
            .cloned()
            .collect()
    }
}
