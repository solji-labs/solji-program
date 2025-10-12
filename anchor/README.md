# Solji Temple - Solana 程序

## 环境配置

### 1. 复制环境变量模板
```bash
cp .env.example .env
```

### 2. 编辑 .env 文件
```bash
# 如果你有 Ankr API Key，可以填入以获得更好的网络性能
ANKR_API_KEY=your_ankr_api_key_here
```

### 3. 安装依赖
```bash
yarn install
```

## 部署和测试

### 使用脚本部署到 devnet
```bash
# 使用默认 RPC
yarn deploy:devnet

# 或者设置 ANKR_API_KEY 环境变量后使用
ANKR_API_KEY=your_key yarn deploy:devnet
```

### 运行测试
```bash
# 初始化寺庙
yarn test:temple-init

# 初始化香型
yarn test:incense-init

# 初始化用户
yarn test:user-init

# 或者手动指定测试
ANKR_API_KEY=your_key ./scripts/test-devnet.sh temple-init
```

### 手动命令
如果你需要手动运行命令，可以这样设置环境变量：

```bash
# 设置环境变量
export ANKR_API_KEY=your_ankr_api_key_here
export ANCHOR_PROVIDER_URL=https://rpc.ankr.com/solana_devnet/$ANKR_API_KEY
export ANCHOR_WALLET=~/.config/solana/id.json

# 然后运行 anchor 命令
anchor deploy
```

## 安全注意事项

- **永远不要**将 `.env` 文件提交到版本控制系统
- API Key 应该保密，不要在公开场合分享
- 使用 `.env.example` 作为模板，但不要在其中包含真实的 API Key
