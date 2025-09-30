use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, commitment_config::CommitmentConfig};
use anchor_lang::AccountDeserialize;
use std::str::FromStr;
// 直接导入链上程序的 crate 和其中的结构体
use temple::{
    state::GlobalStats,      // 假设 GlobalStats 在 state 模块下
    state::UserDonationState,
}; 
use anchor_lang::AccountDeserialize; // 仍然需要这个 Trait
const TEMPLE_PROGRAM_ID: &str = "D9immZaczS2ASFqqSux2iCCAaFat7vcusB1PQ2SW6d95"; 
const GLOBAL_STATS_SEED: &[u8] = b"global_stats"; // 您的 GlobalStats PDA 种子

pub fn get_global_stats_pda(program_id: &Pubkey) -> Pubkey {
    let (pda, _bump) = Pubkey::find_program_address(
        &[GLOBAL_STATS_SEED],
        program_id,
    );
    pda
}

pub async fn fetch_global_stats(rpc_url: &str) -> Result<GlobalStats, Box<dyn std::error::Error>> {
    let client = RpcClient::new(rpc_url.to_string());
    let program_id = Pubkey::from_str(TEMPLE_PROGRAM_ID)?;
    
    // 1. 计算 GlobalStats 的 PDA 地址
    let global_stats_pda = get_global_stats_pda(&program_id);

    // 2. 从 RPC 节点获取账户数据
    let account_data = client.get_account(&global_stats_pda)?;
    
    // 3. 反序列化账户数据
    let mut data_slice: &[u8] = &account_data.data;
    
    // 使用 Anchor 的反序列化方法
    let global_stats = GlobalStats::try_deserialize(&mut data_slice)?;

    tracing::info!("Successfully fetched GlobalStats: Total Donators = {}", global_stats.total_users);
    
    Ok(global_stats)
}