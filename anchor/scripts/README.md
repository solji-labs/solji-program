# 生产环境初始化脚本

此脚本在Solana主网上初始化Temple程序的所有必要配置。

## 前置条件

1. **管理员密钥对**：需要有足够SOL支付交易费用的管理员密钥对
2. **金库地址**：接收捐赠的金库钱包地址
3. **Node.js**：版本16或更高
4. **Anchor CLI**：最新版本

## 设置

### 1. 配置生产环境设置

编辑`init-production.ts`，更新`PRODUCTION_CONFIG`：

```typescript
const PRODUCTION_CONFIG = {
    programId: new PublicKey("D9immZaczS2ASFqqSux2iCCAaFat7vcusB1PQ2SW6d95"),
    adminKeypairPath: "~/.config/solana/admin-keypair.json", // 更新此路径
    treasuryAddress: new PublicKey("YOUR_TREASURY_ADDRESS"), // 更新此地址
    // ... 其余配置
};
```

### 2. 准备管理员钱包

```bash
# 创建或使用现有的管理员密钥对
solana-keygen new --outfile ~/.config/solana/admin-keypair.json

# 为管理员钱包充值SOL（建议至少10 SOL）
solana airdrop 10 <ADMIN_PUBKEY>  # 用于devnet
# 或从其他钱包转入SOL用于mainnet
```

### 3. 将程序部署到主网

```bash
# 更新Anchor.toml用于主网
[provider]
cluster = "https://api.mainnet-beta.solana.com"

# 构建并部署
anchor build
anchor deploy --provider.cluster mainnet
```

## 运行初始化

### Devnet环境（默认）

```bash
cd solji-program/anchor
npx tsx scripts/init-production.ts
# 或
npx tsx scripts/init-production.ts devnet
```

### Mainnet环境

```bash
cd solji-program/anchor
npx tsx scripts/init-production.ts mainnet
```

### 使用Anchor脚本

添加到`Anchor.toml`：

```toml
[scripts]
init-devnet = "npx tsx scripts/init-production.ts devnet"
init-mainnet = "npx tsx scripts/init-production.ts mainnet"
```

然后运行：

```bash
anchor run init-devnet
# 或
anchor run init-mainnet
```

## 脚本执行内容

### 步骤1：创建Temple配置
- 初始化主要的temple配置PDA
- 设置占卜概率
- 配置捐赠等级和奖励
- 创建全局统计账户

### 步骤2：创建NFT铸币
- 为所有6种香类型创建NFT铸币账户：
  1. 清香 (Fresh Incense)
  2. 檀香 (Sandalwood Incense)
  3. 龙涎香 (Ambergris Incense)
  4. 太上灵香 (Supreme Spirit Incense)
  5. 秘制香 (Secret Brew Incense) - 仅通过捐赠获得
  6. 天界香 (Celestial Incense) - 仅通过捐赠获得

### 步骤3：初始化商城配置
- 使用所有香物品设置商城
- 配置价格、库存和可用性

## 配置详情

### 香类型
- **普通香** (ID 1-4)：可以用SOL购买
- **特殊香** (ID 5-6)：仅通过捐赠获得

### 占卜概率
- **普通用户**：标准占卜概率
- **佛像NFT持有者**：提升的占卜概率

### 捐赠等级
4个等级的捐赠奖励，具有递增的功德和香奖励。

## 初始化后操作

成功初始化后：

1. **更新前端**：修改`solana.ts`使用主网集群
2. **部署前端**：更新并部署前端应用
3. **启动索引器**：配置并启动索引器服务
4. **监控**：为程序设置监控

## 故障排除

### 常见问题

1. **资金不足**：确保管理员钱包有足够的SOL支付所有交易
2. **程序未部署**：验证程序ID与已部署程序匹配
3. **密钥对路径**：检查管理员密钥对路径是否正确
4. **网络问题**：确保与主网连接稳定

### 恢复

如果初始化中途失败：

1. 检查哪些步骤已成功完成
2. 脚本设计为跳过已完成的步骤
3. 重新运行脚本 - 它会从中断处继续

## 安全注意事项

- **管理员密钥对**：安全存储，切勿提交到版本控制
- **金库地址**：生产环境使用多签名钱包
- **程序所有权**：初始化后只有管理员可以更新配置
- **备份**：保留所有配置数据的备份

## 更新配置

初始化后，使用这些指令更新设置：

- `update_incense_types`：修改香配置
- `update_donation_levels`：更改捐赠奖励
- `update_fortune_config`：调整占卜概率
- `update_temple_status`：启用/禁用功能

## 支持

初始化脚本问题排查：

1. 管理员钱包余额
2. 网络连接
3. 程序部署状态
4. 脚本中的配置值
