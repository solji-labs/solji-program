
pub use anchor_lang::prelude::*;



/// 全局寺庙状态
/// 存储寺庙的整体信息和统计数据
/// 这是整个Solji应用的核心状态账户
#[account]
#[derive(Debug, InitSpace)]
pub struct TempleState {
    /// 寺庙管理员地址 - 拥有管理权限的地址
    pub authority: Pubkey,

    /// 寺庙资金池地址 - 存储寺庙收入的地址
    pub treasury: Pubkey,
    
    /// 当前寺庙等级 (1-4: 草庙->赤庙->灵殿->赛博神殿)
    /// 影响UI展示和功能解锁
    pub temple_level: u8,
    
    /// 全网累积香火值，用于寺庙升级判断
    /// 所有用户烧香贡献的香火值总和
    pub total_incense_value: u64,
    
    /// 总抽签次数统计
    /// 用于寺庙升级条件判断
    pub total_draws: u64,
    
    /// 总许愿次数统计 (为未来功能预留)
    /// 预留给后续许愿功能使用
    pub total_wishes: u64,
    
    /// 总捐助 SOL 数量 (以lamports为单位)
    /// 所有用户捐助的SOL总和
    pub total_donations: u64,
    
    /// 佛像 NFT 铸造数量统计 (为未来功能预留)
    /// 预留给后续SBT系统使用
    pub buddha_nft_count: u32,
    
    /// 已初始化的香型数量
    /// 用于统计当前可用的香型种类
    pub incense_type_count: u8,
    
    /// 寺庙创建时间戳
    /// 记录寺庙初始化的时间
    pub created_at: i64,
    
    /// 最后更新时间戳
    /// 记录状态最后一次更新的时间
    pub updated_at: i64,
}


impl TempleState {

    /// 定义TempleState账户的种子前缀
    /// 用于生成PDA地址
    pub const SEED_PREFIX: &'static str = "temple_state_v1";

    /// 初始化寺庙状态
    pub fn initialize(&mut self, authority: Pubkey,treasury: Pubkey, current_timestamp: i64) -> Result<()> {

        self.authority = authority;
        self.treasury = treasury;
        self.temple_level = 1;
        self.total_incense_value = 0;
        self.total_draws = 0;
        self.total_wishes = 0;
        self.total_donations = 0;
        self.buddha_nft_count = 0;
        self.incense_type_count = 0;
        self.created_at = current_timestamp;
        self.updated_at = current_timestamp;

        msg!("Temple initialized successfully >>> authority: {}", authority);

        Ok(())
    }


    /// 判断传入的地址是否为寺庙管理员
    pub fn is_authority(&self, authority: Pubkey) -> bool {
        self.authority == authority
    }

    /// 更新寺庙等级
    pub fn update_level(&mut self, new_level: u8) -> Result<()> {
        require!(new_level >= 1 && new_level <= 4, TempleError::InvalidTempleLevel);
        require!(new_level >= self.temple_level, TempleError::TempleCannotDowngrade);
        

        let old_level = self.temple_level;
        self.temple_level = new_level;
        self.updated_at = Clock::get().unwrap().unix_timestamp;

        msg!("Temple level updated from {} to {}", old_level, new_level);

        Ok(())
    }

    /// 增加香火值
    pub fn add_incense_value(&mut self, value: u64) -> Result<()> {

        // 检查香火值是否溢出
        // checked_add()函数用于安全地进行加法运算
        // ok_or()函数用于在加法运算失败时返回错误
        self.total_incense_value = self.total_incense_value
        .checked_add(value)
        .ok_or(TempleError::IncenseValueOverflow)?;

        // Clock::get().unwrap().unix_timestamp获取当前时间戳
        self.updated_at = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

    /// 增加抽签次数
    pub fn draw_fortune(&mut self) -> Result<()> {

        // 检查抽签次数是否溢出 
        self.total_draws = self.total_draws
        .checked_add(1)
        .ok_or(TempleError::DrawCountOverflow)?;
 
        self.updated_at = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

    /// 增加许愿次数
    pub fn create_wish(&mut self) -> Result<()> {

        // 检查许愿次数是否溢出 
        self.total_wishes = self.total_wishes
        .checked_add(1)
        .ok_or(TempleError::WishCountOverflow)?;
 
        self.updated_at = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }

 

    /// 增加捐助金额
    pub fn add_donation(&mut self, value: u64) -> Result<()> {

        // 检查捐助金额是否溢出
        self.total_donations = self.total_donations
        .checked_add(value)
        .ok_or(TempleError::DonationOverflow)?;
 
        self.updated_at = Clock::get().unwrap().unix_timestamp;

        Ok(())
    }


    /// 增加香型数量
    pub fn increment_incense_type_count(&mut self) -> Result<()> {
        require!(self.incense_type_count < 255, TempleError::TooManyIncenseTypes);
        self.incense_type_count = self.incense_type_count.checked_add(1).unwrap();
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    /// 获取寺庙等级名称
    pub fn get_level_name(&self) -> &'static str {
        match self.temple_level {
            1 => "草庙",
            2 => "赤庙",
            3 => "灵殿",
            4 => "赛博神殿",
            _ => "未知等级",
        }
    }


}


#[error_code]
pub enum TempleError {
    /// 许愿次数溢出
    #[msg("Wish count overflow")]
    WishCountOverflow,
    /// 寺庙等级无效
    #[msg("Invalid Temple Level, must be between 1 and 4")]
    InvalidTempleLevel,

    /// 寺庙等级不能降级
    #[msg("Temple level cannot downgrade")]
    TempleCannotDowngrade,


    /// 香火值溢出
    #[msg("Incense value overflow")]
    IncenseValueOverflow,

    /// 抽签次数溢出
    #[msg("Draw count overflow")]
    DrawCountOverflow,

    /// 捐助次数溢出
    #[msg("Donation overflow")]
    DonationOverflow,

    /// 香型数量过多
    #[msg("Too many incense types, max is 255")]
    TooManyIncenseTypes,

    /// 未经授权的寺庙访问
    #[msg("Unauthorized Temple Access")]
    UnauthorizedTempleAccess,

    /// 寺庙已初始化
    #[msg("Temple already initialized")]
    TempleAlreadyInitialized,

    /// 寺庙不存在
    #[msg("Temple not found")]
    TempleNotFound,
}