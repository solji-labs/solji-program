# Solji Temple Program - API 集成文档

> **Program ID**: `81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o`  
> **Network**: Solana Devnet  
> **Version**: 1.0.0

---

## 目录

- [1. 概述](#1-概述)
- [2. 环境配置](#2-环境配置)
- [3. PDA 地址计算](#3-pda-地址计算)
- [4. 核心指令](#4-核心指令)
- [5. 账户结构](#5-账户结构)
- [6. 错误码](#6-错误码)

---

## 1. 概述

Solji Temple Program 是一个基于 Solana 区块链的去中心化寺庙系统，提供以下核心功能：

- **寺庙管理**: 初始化和管理寺庙全局状态
- **用户系统**: 用户状态管理、功德值、香火值追踪
- **香品系统**: 多种香型的购买、烧香、NFT 铸造
- **互动功能**: 抽签、许愿、点赞
- **捐赠系统**: 捐赠获得功德值和徽章 NFT
- **NFT 系统**: 香品 NFT、徽章 NFT、佛像 NFT

---

## 2. 环境配置

### 2.1 安装依赖

```bash
npm install @coral-xyz/anchor @solana/web3.js @solana/spl-token
```

### 2.2 初始化连接

```typescript
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection("https://api.devnet.solana.com", "confirmed");
const programId = new PublicKey("81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o");
const provider = new anchor.AnchorProvider(connection, wallet, {
  commitment: "confirmed",
});
const program = new Program(idl, programId, provider);
```

### 2.3 命令行配置

```bash
solana config set --url https://api.devnet.solana.com
solana config set --keypair ~/.config/solana/id.json
```

---

## 3. PDA 地址计算

### 3.1 寺庙配置 PDA

```typescript
const [templeConfigPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("temple_config_v1")],
  programId
);
```

### 3.2 用户状态 PDA

```typescript
const [userStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("user_state_v1"), userPublicKey.toBuffer()],
  programId
);
```

### 3.3 香型配置 PDA

```typescript
const [incenseTypeConfigPda] = PublicKey.findProgramAddressSync(
  [Buffer.from("incense_type_v1"), Buffer.from([incenseTypeId])],
  programId
);
```

### 3.4 用户香炉状态 PDA

```typescript
const [userIncenseStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("user_incense_v1"), userPublicKey.toBuffer()],
  programId
);
```

### 3.5 许愿 PDA

```typescript
const [wishPda] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("wish_v1"),
    creatorPublicKey.toBuffer(),
    new anchor.BN(wishId).toArrayLike(Buffer, "le", 8)
  ],
  programId
);
```

### 3.6 用户捐赠状态 PDA

```typescript
const [userDonationStatePda] = PublicKey.findProgramAddressSync(
  [Buffer.from("user_donation_v1"), userPublicKey.toBuffer()],
  programId
);
```

---

## 4. 核心指令

详细的指令说明请参考以下文档：

- [寺庙和用户管理指令](./api_temple_user.md)
- [香品系统指令](./api_incense.md)
- [互动功能指令](./api_interactive.md)
- [捐赠和 NFT 指令](./api_donation_nft.md)

---

## 5. 账户结构

### 5.1 TempleConfig (寺庙配置)

```rust
pub struct TempleConfig {
    pub authority: Pubkey,           // 管理员地址
    pub treasury: Pubkey,            // 国库地址
    pub temple_level: u8,            // 寺庙等级
    pub total_incense_value: u64,   // 总香火值
    pub total_draws: u64,            // 总抽签次数
    pub total_wishes: u64,           // 总许愿次数
    pub total_donations: u64,        // 总捐赠金额
    pub total_buddha_nft: u32,       // 佛像NFT总数
    pub incense_type_count: u8,      // 香型数量
    pub created_at: i64,             // 创建时间
    pub updated_at: i64,             // 更新时间
}
```

### 5.2 UserState (用户状态)

```rust
pub struct UserState {
    pub user: Pubkey,                // 用户地址
    pub karma_points: u64,           // 功德值
    pub incense_value: u64,          // 香火值
    pub total_spending: u64,         // 总消费
    pub total_burn_count: u64,       // 总烧香次数
    pub total_draw_count: u64,       // 总抽签次数
    pub total_wish_count: u64,       // 总许愿次数
    pub daily_burn_count: u8,        // 每日烧香次数
    pub daily_draw_count: u8,        // 每日抽签次数
    pub daily_wish_count: u8,        // 每日许愿次数
    pub last_burn_date: i64,         // 最后烧香日期
    pub last_draw_date: i64,         // 最后抽签日期
    pub last_wish_date: i64,         // 最后许愿日期
    pub created_at: i64,             // 创建时间
    pub updated_at: i64,             // 更新时间
}
```

### 5.3 IncenseTypeConfig (香型配置)

```rust
pub struct IncenseTypeConfig {
    pub incense_type_id: u8,         // 香型ID
    pub name: String,                // 名称
    pub description: String,         // 描述
    pub price_per_unit: u64,         // 单价
    pub karma_reward: u32,           // 功德奖励
    pub incense_value: u32,          // 香火值
    pub purchasable_with_sol: bool,  // 是否可用SOL购买
    pub max_buy_per_transaction: u8, // 单次最大购买数
    pub is_active: bool,             // 是否激活
    pub rarity: IncenseRarity,       // 稀有度
    pub nft_collection: Pubkey,      // NFT集合
    pub metadata_uri_template: String, // 元数据URI模板
    pub total_minted: u64,           // 已铸造总数
    pub created_at: i64,             // 创建时间
    pub updated_at: i64,             // 更新时间
}
```

---

## 6. 错误码

### 6.1 通用错误

- `0x1770` (6000): 未授权访问
- `0x1771` (6001): 无效的寺庙配置
- `0x1772` (6002): 无效的国库地址

### 6.2 用户错误

- `0x1774` (6004): 功德值不足
- `0x1775` (6005): 每日烧香次数已用完
- `0x1776` (6006): 每日抽签次数已用完
- `0x1777` (6007): 每日许愿次数已用完

### 6.3 香品错误

- `0x1778` (6008): 香型未激活
- `0x1779` (6009): 香余额不足
- `0x177A` (6010): 购买数量超过限制
- `0x177B` (6011): 支付金额不足

### 6.4 许愿错误

- `0x177C` (6012): 许愿已存在
- `0x177D` (6013): 点赞已存在
- `0x177E` (6014): 无效的创建者

---

## 快速开始

### 1. 初始化寺庙（管理员）

```bash
anchor run temple-init
```

### 2. 初始化用户

```bash
anchor run user-init
```

### 3. 购买香

```bash
anchor run incense-buy
```

### 4. 烧香

```bash
anchor run incense-burn-simplied
```

### 5. 抽签

```bash
anchor run draw-fortune
```

### 6. 许愿

```bash
anchor run wish
```

### 7. 捐赠

```bash
anchor run donation
```

---

## 相关资源

- [GitHub Repository](https://github.com/solji-labs/solji-program)
- [Solana Explorer (Devnet)](https://explorer.solana.com/?cluster=devnet)
- [Anchor Documentation](https://www.anchor-lang.com/)

