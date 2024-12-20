use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;

pub fn create_rpc_client() -> RpcClient {
    dotenv().ok();
    let commitment_config = CommitmentConfig::processed();
    let helius_rpc_url = env::var("HELIUS_RPC_URL").expect("HELIUS_RPC_URL not set");
    RpcClient::new_with_commitment(helius_rpc_url, commitment_config)
}
