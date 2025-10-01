use anchor_lang::prelude::*;

// 标准化香型ID定义
pub const QING_XIANG: u8 = 1; // 清香
pub const TAN_XIANG: u8 = 2; // 檀香
pub const LONG_XIAN_XIANG: u8 = 3; // 龙涎香
pub const TAI_SHANG_XIANG: u8 = 4; // 太上灵香
pub const MI_ZHI_XIANG: u8 = 5; // 秘制香（捐助解锁）
pub const TIAN_JIE_XIANG: u8 = 6; // 天界香（捐助解锁）

// 香品稀有度定义
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, AnchorSerialize, AnchorDeserialize)]
pub enum IncenseRarity {
    Common = 1,    // 普通
    Rare = 2,      // 稀有
    Epic = 3,      // 史诗
    Legendary = 4, // 传说
}

/// 实现 Space trait 以便在账户中正确序列化
impl anchor_lang::Space for IncenseRarity {
    const INIT_SPACE: usize = 1; // 枚举在序列化时占用1个字节
}

impl Default for IncenseRarity {
    fn default() -> Self {
        Self::Common
    }
}

/// 香型配置
/// 管理端定义每种香的基本属性和价格
#[account]
#[derive(Debug, InitSpace)]
pub struct IncenseTypeConfig {
    /// 香型ID (使用上述常量)
    pub incense_type_id: u8,

    /// 香的名称 (限制32字符)
    #[max_len(32)]
    pub name: String,

    /// 香的描述 (限制128字符)
    #[max_len(128)]
    pub description: String,

    /// 购买价格 (lamports per 根)
    pub price_per_unit: u64,

    /// 烧香获得的功德值
    pub karma_reward: u32,

    /// 烧香贡献的香火值
    pub incense_value: u32,

    /// 是否可以通过SOL购买
    pub purchasable_with_sol: bool,

    /// 每次最大购买数量
    pub max_purchase_per_transaction: u8,

    /// 是否激活此香型
    pub is_active: bool,

    /// 香的稀有度
    pub rarity: IncenseRarity,

    /// NFT Collection 地址
    pub nft_collection: Pubkey,

    /// NFT 元数据 URI 模板 (限制200字符)
    #[max_len(200)]
    pub metadata_uri_template: String,

    /// 该香型已铸造的总数量（解决NFT命名重复问题）
    pub total_minted: u64,

    /// 创建时间戳
    pub created_at: i64,

    /// 最后更新时间戳
    pub updated_at: i64,
}


impl IncenseTypeConfig {

    /// PDA种子前缀
    pub const SEED_PREFIX: &'static str = "incense_type_v1";

        /// 最大名称长度 (已通过 #[max_len] 宏定义)
        pub const MAX_NAME_LEN: usize = 32;
    
        /// 最大描述长度 (已通过 #[max_len] 宏定义)
        pub const MAX_DESCRIPTION_LEN: usize = 128;
        
        /// 最大URI模板长度 (已通过 #[max_len] 宏定义)
        pub const MAX_URI_TEMPLATE_LEN: usize = 200;

    /// 初始化香型配置
    pub fn initialize(
        &mut self, 
        params: InitializeIncenseTypeParams,
        current_timestamp: i64
    ) -> Result<()> {
        // 验证参数
        require!(params.incense_type_id <= 5, IncenseError::InvalidIncenseTypeId);
        require!(!params.name.is_empty(), IncenseError::EmptyIncenseName);
        require!(params.name.len() <= Self::MAX_NAME_LEN, IncenseError::IncenseNameTooLong);
        require!(params.description.len() <= Self::MAX_DESCRIPTION_LEN, IncenseError::DescriptionTooLong);
        require!(params.metadata_uri_template.len() <= Self::MAX_URI_TEMPLATE_LEN, IncenseError::UriTemplateTooLong);
        require!(params.price_per_unit > 0, IncenseError::InvalidPrice);
        require!(params.karma_reward > 0, IncenseError::InvalidKarmaReward);
        require!(params.incense_value > 0, IncenseError::InvalidIncenseValue);
        require!(params.max_purchase_per_transaction > 0 && params.max_purchase_per_transaction <= 10, 
                IncenseError::InvalidMaxPurchase);

        // 设置基本属性
        self.incense_type_id = params.incense_type_id;
        self.name = params.name;
        self.description = params.description;
        self.price_per_unit = params.price_per_unit;
        self.karma_reward = params.karma_reward;
        self.incense_value = params.incense_value;
        self.purchasable_with_sol = params.purchasable_with_sol;
        self.max_purchase_per_transaction = params.max_purchase_per_transaction;
        self.is_active = params.is_active;
        self.rarity = params.rarity;
        self.nft_collection = params.nft_collection;
        self.metadata_uri_template = params.metadata_uri_template;
        
        // 初始化统计数据
        self.total_minted = 0;
        self.created_at = current_timestamp;
        self.updated_at = current_timestamp;

        msg!("Incense type '{}' initialized successfully", self.name);
        Ok(())
    }
    


    /// 增加铸造数量
    pub fn increment_minted_count(&mut self, count: u64) -> Result<()> {

        self.total_minted = self.total_minted.checked_add(count).ok_or(IncenseError::MintedCountOverflow)?;
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    /// 检查香型是否激活
    pub fn is_active(&self) -> bool {
        self.is_active
    }

    /// 获取下一个序列号
    pub fn get_next_sequence(&self) -> u64 {
        self.total_minted + 1
    }

    /// 格式化NFT名称
    pub fn format_nft_name(&self, sequence: u64) -> String {
        // {:06} 表示6位数字，不足6位前面补0
        format!("{} #{:06}", self.name, sequence)
    }

}




/// 初始化香型的参数结构
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InitializeIncenseTypeParams {
    /// 香型ID
    pub incense_type_id: u8,
    /// 香的名称
    pub name: String,
    /// 香的描述
    pub description: String,
    /// 购买价格 (lamports)
    pub price_per_unit: u64,
    /// 烧香获得的功德值
    pub karma_reward: u32,
    /// 烧香贡献的香火值
    pub incense_value: u32,
    /// 是否可以通过SOL购买
    pub purchasable_with_sol: bool,
    /// 每次最大购买数量
    pub max_purchase_per_transaction: u8,
    /// 是否激活此香型
    pub is_active: bool,
    /// 香的稀有度
    pub rarity: IncenseRarity,
    /// NFT Collection 地址
    pub nft_collection: Pubkey,
    /// NFT 元数据 URI 模板
    pub metadata_uri_template: String,
}



/// 香型相关错误定义
#[error_code]
pub enum IncenseError {
    #[msg("Invalid incense type ID, must be between 0 and 5")]
    InvalidIncenseTypeId,
    
    #[msg("Incense name cannot be empty")]
    EmptyIncenseName,
    
    #[msg("Incense name too long, maximum 32 characters")]
    IncenseNameTooLong,
    
    #[msg("Description too long, maximum 128 characters")]
    DescriptionTooLong,
    
    #[msg("URI template too long, maximum 200 characters")]
    UriTemplateTooLong,
    
    #[msg("Price must be greater than 0")]
    InvalidPrice,
    
    #[msg("Karma reward must be greater than 0")]
    InvalidKarmaReward,
    
    #[msg("Incense value must be greater than 0")]
    InvalidIncenseValue,
    
    #[msg("Max purchase per transaction must be between 1 and 10")]
    InvalidMaxPurchase,
    
    #[msg("Minted count overflow")]
    MintedCountOverflow,
    
    #[msg("Incense type already exists")]
    IncenseTypeAlreadyExists,
    
    #[msg("Incense type not found")]
    IncenseTypeNotFound,
    
    #[msg("Incense type not active")]
    IncenseTypeNotActive,
}