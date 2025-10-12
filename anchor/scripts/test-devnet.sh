#!/bin/bash

# Solana Devnet 测试脚本
# 使用环境变量来配置 RPC 端点

# 设置默认值
DEFAULT_RPC="https://api.devnet.solana.com"
ANKR_RPC_BASE="https://rpc.ankr.com/solana_devnet"

# 检查是否设置了 ANKR_API_KEY 环境变量
if [ -n "$ANKR_API_KEY" ]; then
    echo "使用 Ankr RPC 端点进行测试..."
    RPC_URL="${ANKR_RPC_BASE}/${ANKR_API_KEY}"
else
    echo "使用默认 Solana RPC 端点进行测试..."
    RPC_URL="$DEFAULT_RPC"
fi

echo "RPC URL: $RPC_URL"

# 设置环境变量
export ANCHOR_PROVIDER_URL="$RPC_URL"
export ANCHOR_WALLET="$HOME/.config/solana/id.json"

# 运行指定的测试
if [ -n "$1" ]; then
    echo "运行测试: $1"
    npx mocha --require tsx "tests/$1.test.ts" --timeout 30000
else
    echo "请指定要运行的测试，例如："
    echo "./scripts/test-devnet.sh temple-init"
    echo "./scripts/test-devnet.sh incense-init"
    echo "./scripts/test-devnet.sh user-init"
fi
