# Solji 寺庙项目技术文档

## 项目概述

Solji 是一个构建于 Solana 区块链的 Web3 神坛场域，通过香火、许愿、抽签等充满仪式感的行为，将佛教文化与 meme 文化融合，打造链上第一座可交互、可升级、可供养的加密寺庙。

### 核心功能
- **寺庙初始化**：创建和管理寺庙全局状态
- **香型管理**：定义和配置不同类型的香品
- **买香系统**：用户购买各种香型的功能
- **烧香系统**：用户烧香获得功德值和香火值
- **抽签功能**：链上随机抽签获得签文
- **许愿功能**：用户许愿并存储到链上
- **捐助系统**：用户捐助获得等级和特权
- **NFT 系统**：佛像 SBT、功德香 NFT、寺庙助章 NFT

## 技术架构

### 技术栈
- **区块链**：Solana
- **智能合约框架**：Anchor 0.31.0
- **语言**：Rust
- **NFT 标准**：Metaplex Token Metadata
- **部署环境**：Devnet/Localnet

### 程序 ID
- **程序地址**：`81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o`
- **管理员地址**：
  - Devnet: `DRayqG9RXYi8WHgWEmRQGrUWRWbhjYWYkCRJDd6JBBak`
  - Localnet: `FcKkQZRxD5P6JwGv58vGRAcX3CkjbX8oqFiygz6ohceU`

## 核心业务逻辑

### 1. 寺庙初始化 (init_temple)

**功能**：创建寺庙全局状态，设置管理员和资金池地址

**状态结构**：
```rust
pub struct TempleConfig {
    pub authority: Pubkey,           // 寺庙管理员
    pub treasury: Pubkey,            // 资金池地址
    pub temple_level: u8,            // 寺庙等级 (1-4)
    pub total_incense_value: u64,    // 全网累积香火值
    pub total_draws: u64,            // 总抽签次数
    pub total_wishes: u64,           // 总许愿次数
    pub total_donations: u64,        // 总捐助次数
    pub total_buddha_nft: u32,       // 佛像 NFT 数量
    pub incense_type_count: u8,      // 香型数量
    pub created_at: i64,             // 创建时间
    pub updated_at: i64,             // 更新时间
}
```

**寺庙等级系统**：
- Lv.1 草庙：初始状态
- Lv.2 赤庙：香火值 ≥ 10000，抽签 ≥ 5000，许愿 ≥ 3000
- Lv.3 灵殿：香火值 ≥ 500000，签文 NFT ≥ 30000
- Lv.4 赛博神殿：香火值 ≥ 2000，签文 NFT ≥ 100000

### 2. 香型初始化 (init_incense_type)

**功能**：管理员创建不同类型的香品配置

**香型定义**：
```rust
pub const QING_XIANG: u8 = 1;      // 清香 - 0.01 SOL
pub const TAN_XIANG: u8 = 2;       // 檀香 - 0.05 SOL
pub const LONG_XIAN_XIANG: u8 = 3; // 龙涎香 - 0.1 SOL
pub const TAI_SHANG_XIANG: u8 = 4; // 太上灵香 - 0.3 SOL
pub const MI_ZHI_XIANG: u8 = 5;    // 秘制香 - 捐助解锁
pub const TIAN_JIE_XIANG: u8 = 6;  // 天界香 - 捐助解锁
```

**香型配置结构**：
```rust
pub struct IncenseTypeConfig {
    pub incense_type_id: u8,         // 香型 ID
    pub name: String,                // 香名称
    pub description: String,         // 描述
    pub price_per_unit: u64,         // 单价 (lamports)
    pub karma_reward: u32,           // 功德值奖励
    pub incense_value: u32,          // 香火值贡献
    pub purchasable_with_sol: bool,  // 是否可 SOL 购买
    pub max_buy_per_transaction: u8, // 单次最大购买量
    pub is_active: bool,             // 是否激活
    pub rarity: IncenseRarity,       // 稀有度
    pub nft_collection: Pubkey,      // NFT 集合地址
    pub metadata_uri_template: String, // 元数据模板
    pub total_minted: u64,           // 已铸造数量
}
```

### 3. 买香功能 (buy_incense)

**功能**：用户购买香品，支付 SOL 获得香的余额

**核心流程**：
1. 验证购买参数（数量、价格、香型状态）
2. 检查用户支付金额是否足够
3. 转账 SOL 到寺庙资金池
4. 增加用户香品余额
5. 更新香型铸造统计
6. 记录用户消费数据

**用户香品状态**：
```rust
pub struct UserIncenseState {
    pub user: Pubkey,
    pub incense_having_balances: [IncenseBalance; 6],  // 拥有的香
    pub incense_burned_balances: [IncenseBalance; 6],  // 已烧的香
    pub incense_total_balances: [IncenseBalance; 6],   // 总计香品
    pub last_active_at: i64,
}
```

### 4. 烧香功能 (burn_incense)

**功能**：用户消耗香品进行烧香，获得功德值和香火值，同时铸造功德香 NFT

**核心流程**：
1. 检查用户香品余额是否足够
2. 检查每日烧香次数限制（基础 10 次 + 捐助解锁）
3. 消耗用户香品余额
4. 增加用户功德值和香火值
5. 增加寺庙全局香火值
6. 铸造功德香 NFT 给用户作为凭证

**用户状态管理**：
```rust
pub struct UserState {
    pub user: Pubkey,
    pub karma_points: u64,           // 功德值
    pub total_incense_value: u64,    // 贡献香火值
    pub total_sol_spent: u64,        // 总消费
    pub donation_unlocked_burns: u8, // 捐助解锁烧香次数
    pub daily_burn_count: u8,        // 今日烧香次数
    pub daily_draw_count: u8,        // 今日抽签次数
    pub daily_wish_count: u8,        // 今日许愿次数
    pub last_action_day: u16,        // 上次操作日期
}
```

**每日限制系统**：
- 基础烧香次数：10 次/天
- 通过捐助可解锁额外次数
- 每日 0 点自动重置计数

## Go 语言扫链功能设计

### 架构设计

基于你的需求，我设计了一个 Go 语言的扫链服务来监控 Solji 寺庙的链上活动：

```go
// 核心数据结构
type TempleEvent struct {
    EventType   string    `json:"event_type"`
    Signature   string    `json:"signature"`
    BlockTime   time.Time `json:"block_time"`
    User        string    `json:"user"`
    Data        string    `json:"data"`
}

type TempleStats struct {
    Date            time.Time `json:"date"`
    TotalBurns      int64     `json:"total_burns"`
    TotalPurchases  int64     `json:"total_purchases"`
    ActiveUsers     int64     `json:"active_users"`
    TotalRevenue    int64     `json:"total_revenue"`
    TempleLevel     int       `json:"temple_level"`
}
```

### 监控功能模块

1. **寺庙初始化监控**
   - 监控 `TempleInitEvent` 事件
   - 记录寺庙创建时间和管理员

2. **香型初始化监控**
   - 监控 `IncenseInitEvent` 事件
   - 记录新香型的创建和配置

3. **买香活动监控**
   - 监控 `BuyIncenseEvent` 事件
   - 统计用户购买行为和收入数据

4. **烧香活动监控**
   - 监控烧香交易和 NFT 铸造
   - 统计每日烧香数据和活跃用户

### 数据库设计

```sql
-- 寺庙状态表
CREATE TABLE temple_config (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    authority VARCHAR(44) NOT NULL,
    treasury VARCHAR(44) NOT NULL,
    temple_level TINYINT DEFAULT 1,
    total_incense_value BIGINT DEFAULT 0,
    total_draws BIGINT DEFAULT 0,
    total_wishes BIGINT DEFAULT 0,
    total_donations BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- 香型配置表
CREATE TABLE incense_types (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    incense_type_id TINYINT NOT NULL,
    name VARCHAR(32) NOT NULL,
    price_per_unit BIGINT NOT NULL,
    karma_reward INT NOT NULL,
    incense_value INT NOT NULL,
    is_active BOOLEAN DEFAULT TRUE,
    total_minted BIGINT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE KEY uk_incense_type_id (incense_type_id)
);

-- 用户状态表
CREATE TABLE user_states (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    user_address VARCHAR(44) NOT NULL,
    karma_points BIGINT DEFAULT 0,
    total_incense_value BIGINT DEFAULT 0,
    total_sol_spent BIGINT DEFAULT 0,
    total_burn_count INT DEFAULT 0,
    total_draw_count INT DEFAULT 0,
    total_wish_count INT DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY uk_user_address (user_address)
);

-- 买香记录表
CREATE TABLE buy_incense_events (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    signature VARCHAR(88) NOT NULL,
    user_address VARCHAR(44) NOT NULL,
    incense_type_id TINYINT NOT NULL,
    quantity TINYINT NOT NULL,
    unit_price BIGINT NOT NULL,
    total_amount BIGINT NOT NULL,
    block_time TIMESTAMP NOT NULL,
    slot BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE KEY uk_signature (signature)
);

-- 烧香记录表
CREATE TABLE burn_incense_events (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    signature VARCHAR(88) NOT NULL,
    user_address VARCHAR(44) NOT NULL,
    incense_type_id TINYINT NOT NULL,
    amount TINYINT NOT NULL,
    karma_gained INT NOT NULL,
    incense_value_gained INT NOT NULL,
    nft_mint VARCHAR(44),
    block_time TIMESTAMP NOT NULL,
    slot BIGINT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE KEY uk_signature (signature)
);

-- 每日统计表
CREATE TABLE daily_stats (
    id BIGINT PRIMARY KEY AUTO_INCREMENT,
    date DATE NOT NULL,
    total_burns INT DEFAULT 0,
    total_purchases INT DEFAULT 0,
    active_users INT DEFAULT 0,
    total_revenue BIGINT DEFAULT 0,
    new_users INT DEFAULT 0,
    temple_level TINYINT DEFAULT 1,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY uk_date (date)
);
```

### API 接口设计

```go
// 统计接口
GET /api/stats/daily?date=2024-10-12
GET /api/stats/temple
GET /api/stats/users/active?period=7d

// 用户接口
GET /api/users/{address}/profile
GET /api/users/{address}/incense-balance
GET /api/users/{address}/activities

// 寺庙接口
GET /api/temple/config
GET /api/temple/incense-types
GET /api/temple/leaderboard
```

## 扫链实现要点

### 1. 事件监听
- 监听程序 `81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o` 的所有交易
- 解析 Anchor 事件日志
- 处理交易确认和重组

### 2. 数据同步
- 实时同步链上状态到数据库
- 处理数据一致性和幂等性
- 支持历史数据回填

### 3. 统计计算
- 每日定时计算统计数据
- 活跃用户识别（24小时内有操作）
- 收入统计和趋势分析

### 4. 监控告警
- 异常交易监控
- 系统健康检查
- 数据同步状态监控

## 部署和运维

### 环境配置
- Solana RPC 节点连接
- MySQL 数据库
- Redis 缓存（可选）
- 监控和日志系统

### 扩展功能
- 抽签功能监控
- 许愿功能监控  
- 捐助功能监控
- NFT 交易监控
- 寺庙升级事件监控

---

*文档版本：v1.0*  
*最后更新：2024年10月*
