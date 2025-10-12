#!/bin/bash

# Solana Devnet 部署脚本
# 使用环境变量来配置 RPC 端点

# 设置默认值
DEFAULT_RPC="https://api.devnet.solana.com"
ANKR_RPC_BASE="https://rpc.ankr.com/solana_devnet"

# 检查是否设置了 ANKR_API_KEY 环境变量
if [ -n "$ANKR_API_KEY" ]; then
    echo "使用 Ankr RPC 端点..."
    RPC_URL="${ANKR_RPC_BASE}/${ANKR_API_KEY}"
else
    echo "使用默认 Solana RPC 端点..."
    RPC_URL="$DEFAULT_RPC"
fi

echo "RPC URL: $RPC_URL"

# 设置环境变量
export ANCHOR_PROVIDER_URL="$RPC_URL"
export ANCHOR_WALLET="$HOME/.config/solana/id.json"

# 执行部署
echo "开始部署到 devnet..."
anchor deploy --provider.cluster "$RPC_URL"

echo "部署完成！"
