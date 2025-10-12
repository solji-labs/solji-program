# Solji Temple 项目 Solana Dev 环境部署指南

> 本指南基于实际部署和调试经验编写，包含所有已验证的步骤和解决方案

## 项目概述

Solji Temple 是一个基于 Solana 区块链的去中心化寺庙应用，使用 Anchor 框架开发。项目实现了数字化的寺庙功能，包括烧香、抽签、许愿、捐赠等传统寺庙活动的区块链版本。

### 项目架构

- **程序名称**: Temple
- **程序 ID**: `81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o`
- **框架**: Anchor v0.31.0
- **语言**: Rust + TypeScript
- **网络**: Solana Devnet

### 核心功能

- 🏛️ **寺庙管理**: 寺庙配置和状态管理
- 🔥 **烧香系统**: 多种香型的购买和燃烧
- 🎯 **抽签功能**: 基于随机数的运势抽签
- 💝 **许愿系统**: 匿名或实名许愿功能
- 💰 **捐赠机制**: SOL 捐赠和奖励系统
- 🎨 **NFT 铸造**: 佛像 NFT 和香型 NFT

## 环境准备

### 1. 系统要求

- **操作系统**: macOS, Linux, 或 Windows (WSL)
- **Node.js**: v18+ 
- **Rust**: 1.70+
- **Solana CLI**: v1.18+
- **Anchor CLI**: v0.31.0
- **Yarn**: 推荐使用 Yarn 作为包管理器

### 2. 安装依赖工具

#### 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustup component add rustfmt
rustup update
```

#### 安装 Solana CLI

```bash
sh -c "$(curl -sSfL https://release.solana.com/v1.18.22/install)"
export PATH="~/.local/share/solana/install/active_release/bin:$PATH"
```

#### 安装 Anchor CLI

```bash
npm install -g @coral-xyz/anchor-cli@0.31.0
```

#### 验证安装

```bash
solana --version
anchor --version
node --version
cargo --version
yarn --version
```

### 3. 配置 Solana 开发环境

#### 生成钱包密钥对

```bash
solana-keygen new --outfile ~/.config/solana/id.json
```

#### 配置网络为 devnet

```bash
solana config set --url https://api.devnet.solana.com
```

#### 获取测试代币

```bash
solana airdrop 2
```

#### 验证配置

```bash
solana config get
solana balance
solana address
```

## 项目设置和配置

### 1. 项目结构

```
solji-program/
├── anchor/
│   ├── Anchor.toml          # Anchor 配置文件
│   ├── Cargo.toml           # Rust 工作空间配置
│   ├── package.json         # Node.js 依赖和脚本
│   ├── programs/
│   │   └── temple/          # 主程序代码
│   ├── tests/               # 测试文件
│   ├── scripts/             # 部署和测试脚本
│   └── .env.example         # 环境变量模板
└── docs/                    # 文档目录
```

### 2. 环境变量配置（重要！）

为了安全地管理 API Key，项目使用环境变量：

#### 复制环境变量模板

```bash
cd anchor
cp .env.example .env
```

#### 编辑 .env 文件

```bash
# .env 文件内容
ANKR_API_KEY=your_ankr_api_key_here
SOLANA_DEVNET_RPC=https://api.devnet.solana.com
SOLANA_WALLET_PATH=~/.config/solana/id.json
```

> ⚠️ **安全提醒**: `.env` 文件已在 `.gitignore` 中，不会被提交到版本控制系统

### 3. 安装项目依赖

```bash
cd anchor
yarn install
```

## 网络连接问题解决方案

### 常见网络问题

在部署过程中可能遇到以下网络问题：

1. **官方 RPC 超时**: `https://api.devnet.solana.com` 连接不稳定
2. **WebSocket 不支持**: 某些 RPC 端点不支持 WebSocket 连接
3. **空投限制**: 第三方 RPC 可能不支持空投功能

### 推荐解决方案

#### 方案 1: 使用 Ankr RPC（推荐）

获取 Ankr API Key 并设置环境变量：

```bash
export ANKR_API_KEY=your_ankr_api_key
```

#### 方案 2: 使用脚本自动切换

项目提供了智能脚本，会自动选择最佳 RPC 端点：

```bash
# 脚本会自动检测 ANKR_API_KEY 环境变量
# 如果存在，使用 Ankr RPC；否则使用官方 RPC
./scripts/deploy-devnet.sh
```

## 构建和部署流程

### 1. 检查配置文件

确保 `Anchor.toml` 配置正确：

```toml
[toolchain]
package_manager = "yarn"

[features]
resolution = true
skip-lint = false

[programs.localnet]
temple = "81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o"

[programs.devnet]
temple = "81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o"

[provider]
cluster = "https://api.devnet.solana.com"
wallet = "~/.config/solana/id.json"

[test]
validator = { url = "https://api.devnet.solana.com" }
startup_wait = 10000
```

### 2. 构建程序

```bash
# 清理之前的构建
anchor clean

# 构建程序 (使用 devnet 特性)
anchor build -- --features devnet
```

### 3. 检查程序 ID 一致性

```bash
# 检查生成的程序 ID
anchor keys list

# 输出应该显示:
# temple: 81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o
```

### 4. 部署到 Devnet

#### 方法 1: 使用部署脚本（推荐）

```bash
# 使用默认 RPC
yarn deploy:devnet

# 或使用 Ankr RPC（如果设置了 API Key）
ANKR_API_KEY=your_key yarn deploy:devnet
```

#### 方法 2: 手动部署

```bash
# 设置环境变量（如果使用 Ankr）
export ANKR_API_KEY=your_ankr_api_key
export ANCHOR_PROVIDER_URL=https://rpc.ankr.com/solana_devnet/$ANKR_API_KEY

# 部署
anchor deploy
```

### 5. 验证部署成功

```bash
# 检查程序账户
solana account 81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o --url devnet

# 输出应该显示程序已部署且可执行
# Executable: true
# Owner: BPFLoaderUpgradeab1e11111111111111111111111
```

## 程序初始化

### 1. 寺庙初始化

```bash
# 使用测试脚本初始化寺庙
ANKR_API_KEY=your_key yarn test:temple-init

# 或手动运行
ANKR_API_KEY=your_key ./scripts/test-devnet.sh temple-init
```

**成功输出示例**:
```
Temple Program Test Suite
========================
Authority:  6b8998PfGXtHXCuMdsBqJQwMkcCpVkGRWtk5diDhh3v7
Temple State PDA:  FmxkrARUYSErsD7SCgnTdLLhjMguHy1KXqJSuYUtpNfk
🔍 Temple already exists, reading existing data...
✅ Data Verification:
Authority matches: true
Temple level: 1
```

### 2. 用户初始化

由于网络限制，推荐使用无空投版本的测试：

```bash
# 使用现有钱包初始化用户（无需空投）
ANKR_API_KEY=your_key yarn test:user-init-no-airdrop
```

**成功输出示例**:
```
User Program Test Suite (No Airdrop)
=====================================
User:  6b8998PfGXtHXCuMdsBqJQwMkcCpVkGRWtk5diDhh3v7
User Balance:  1.99032112 SOL
🚀 Initializing new user state PDA...
✅ User state PDA initialization completed!
```

## 测试验证

### 可用的测试命令

```bash
# 寺庙初始化测试
yarn test:temple-init

# 用户初始化测试（无空投版本）
yarn test:user-init-no-airdrop

# 香型初始化测试
yarn test:incense-init

# 通用测试脚本
./scripts/test-devnet.sh <test-name>
```

### 测试环境变量设置

所有测试都需要设置正确的环境变量：

```bash
# 方法 1: 临时设置
ANKR_API_KEY=your_key yarn test:temple-init

# 方法 2: 导出环境变量
export ANKR_API_KEY=your_key
export ANCHOR_PROVIDER_URL=https://rpc.ankr.com/solana_devnet/$ANKR_API_KEY
export ANCHOR_WALLET=~/.config/solana/id.json
yarn test:temple-init
```

## 故障排除

### 1. 网络连接问题

**问题**: `ConnectTimeoutError` 或 `operation timed out`

**解决方案**:
```bash
# 使用 Ankr RPC
ANKR_API_KEY=your_key yarn deploy:devnet

# 或尝试其他 RPC 端点
export ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
```

### 2. WebSocket 连接问题

**问题**: `WebSocket is disabled` 错误

**解决方案**:
```bash
# 使用支持 WebSocket 的 RPC 端点
# 避免使用某些第三方 RPC 进行部署
anchor deploy --provider.cluster https://api.devnet.solana.com
```

### 3. 余额不足

**问题**: `insufficient funds for spend`

**解决方案**:
```bash
# 获取更多测试代币
solana airdrop 2 --url devnet

# 或使用在线水龙头
# https://solfaucet.com/
# https://faucet.quicknode.com/solana/devnet
```

### 4. 交易超时

**问题**: `Transaction was not confirmed in 30.00 seconds`

**解决方案**:
```bash
# 检查交易是否实际成功
solana confirm <transaction-signature> --url devnet

# 增加超时时间
npx mocha --require tsx tests/temple-init.test.ts --timeout 60000
```

### 5. 空投失败

**问题**: `airdrop request failed` 或 `Invalid request`

**解决方案**:
```bash
# 使用无空投版本的测试
yarn test:user-init-no-airdrop

# 或使用在线水龙头获取 SOL
```

## 安全最佳实践

### 1. 环境变量管理

```bash
# ✅ 正确做法：使用环境变量
export ANKR_API_KEY=your_key

# ❌ 错误做法：硬编码在配置文件中
# cluster = "https://rpc.ankr.com/solana_devnet/your_key"
```

### 2. 文件安全

确保以下文件不被提交到版本控制：

```gitignore
# .gitignore 应包含
.env
.env.local
**/*-keypair.json
**/deploy/*-keypair.json
```

### 3. 密钥管理

- **永远不要**将私钥或 API Key 提交到 GitHub
- 使用 `.env.example` 作为模板
- 定期轮换 API Key
- 为生产环境使用硬件钱包

## 部署检查清单

- [ ] ✅ 安装所有必需工具（Rust, Solana CLI, Anchor CLI）
- [ ] ✅ 配置 Solana CLI 连接到 devnet
- [ ] ✅ 生成钱包并获取测试 SOL
- [ ] ✅ 设置环境变量（API Key）
- [ ] ✅ 安装项目依赖 (`yarn install`)
- [ ] ✅ 构建程序 (`anchor build -- --features devnet`)
- [ ] ✅ 验证程序 ID 一致性 (`anchor keys list`)
- [ ] ✅ 部署程序到 devnet
- [ ] ✅ 验证程序部署成功
- [ ] ✅ 初始化寺庙状态
- [ ] ✅ 初始化用户状态
- [ ] ✅ 运行核心功能测试
- [ ] 🔄 设置监控和日志记录
- [ ] 📝 文档化 API 和使用方法

## 项目脚本说明

### package.json 脚本

```json
{
  "scripts": {
    "deploy:devnet": "./scripts/deploy-devnet.sh",
    "test:devnet": "./scripts/test-devnet.sh",
    "test:temple-init": "./scripts/test-devnet.sh temple-init",
    "test:incense-init": "./scripts/test-devnet.sh incense-init",
    "test:user-init": "./scripts/test-devnet.sh user-init",
    "test:user-init-no-airdrop": "./scripts/test-devnet.sh user-init-no-airdrop"
  }
}
```

### 自定义脚本

- `scripts/deploy-devnet.sh`: 智能部署脚本，自动选择最佳 RPC
- `scripts/test-devnet.sh`: 测试脚本，支持环境变量配置

## 监控和维护

### 1. 程序监控

```bash
# 监控程序日志
solana logs 81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o --url devnet

# 检查程序账户状态
solana account 81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o --url devnet --output json
```

### 2. 账户状态查询

```typescript
// 查询寺庙配置
const templeConfig = await program.account.templeConfig.fetch(templeConfigPda);

// 查询用户状态
const userState = await program.account.userState.fetch(userStatePda);

// 查询香型配置
const incenseConfig = await program.account.incenseTypeConfig.fetch(incenseConfigPda);
```

## 下一步开发

部署成功后，您可以：

1. **🎨 前端集成**: 使用生成的客户端代码构建 Web 应用
2. **⚡ 功能扩展**: 添加新的寺庙功能或改进现有功能
3. **🔧 性能优化**: 分析和优化程序性能
4. **🚀 主网准备**: 为生产环境做安全审计和优化
5. **📱 移动应用**: 开发移动端 DApp

## 常用命令速查

```bash
# 快速部署
ANKR_API_KEY=your_key yarn deploy:devnet

# 快速测试
ANKR_API_KEY=your_key yarn test:temple-init
ANKR_API_KEY=your_key yarn test:user-init-no-airdrop

# 检查状态
solana balance --url devnet
solana account 81BWs7RGtN2EEvaGWZe8EQ8nhswHTHVzYUn5iPFoRr9o --url devnet

# 构建和清理
anchor clean && anchor build -- --features devnet

# 验证程序 ID
anchor keys list
```

## 支持和资源

- 📖 [Solana 官方文档](https://docs.solana.com/)
- ⚓ [Anchor 框架文档](https://www.anchor-lang.com/)
- 🛠️ [Solana 开发者工具](https://solana.com/developers)
- 🌐 [Solana Explorer](https://explorer.solana.com/?cluster=devnet)
- 💧 [Solana Devnet 水龙头](https://faucet.solana.com/)

---

**📝 文档版本**: v2.0 - 基于实际部署验证  
**🕒 最后更新**: 2025-10-12  
**⚠️ 注意**: 这是开发环境部署指南。生产环境部署需要额外的安全考虑和配置。