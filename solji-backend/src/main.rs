mod indexer; // 声明 indexer 模块

use axum::{routing::get, Router};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// Health Check 路由处理函数
async fn health_check() -> String {
    "Backend is running.".to_string()
}

// 异步任务：Indexer 主循环（MVP 仅抓取一次）
async fn indexer_task() {
    // 假设 RPC URL 从环境变量中获取
    let rpc_url = std::env::var("SOLANA_RPC_URL").unwrap_or_else(|_| "http://127.0.0.1:8899".to_string());

    tracing::info!("Indexer started. Connecting to RPC: {}", rpc_url);

    match indexer::fetcher::fetch_global_stats(&rpc_url).await {
        Ok(stats) => {
            tracing::info!("Global Stats fetched: Total Donations: {}", stats.total_donations_lamports);
            // TODO: 在此处加入将 stats 存入数据库的逻辑
        }
        Err(e) => {
            tracing::error!("Failed to fetch Global Stats: {:?}", e);
        }
    }
    // TODO: 在实际应用中，这里应该是一个循环或 WebSocket 监听
}

#[tokio::main]
async fn main() {
    // 初始化日志系统
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Solji Backend Server...");
    
    // 1. 启动 Indexer 任务
    tokio::spawn(indexer_task());

    // 2. 启动 API 服务器
    let app = Router::new()
        .route("/health", get(health_check));
        // TODO: 添加 /api/v1/leaderboard 等路由

    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let listener = match TcpListener::bind(format!("0.0.0.0:{}", port)).await {
        Ok(l) => l,
        Err(e) => {
            tracing::error!("Failed to bind port {}: {}", port, e);
            return;
        }
    };

    tracing::info!("API Server listening on port {}", port);

    axum::serve(listener, app)
        .await
        .unwrap();
}