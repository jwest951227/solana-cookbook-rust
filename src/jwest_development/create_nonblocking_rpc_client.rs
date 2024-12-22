use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

pub fn create_nonblocking_rpc_client(
    helius_rpc_https_url: String,
    commitment_config: CommitmentConfig,
) -> RpcClient {
    RpcClient::new_with_commitment(helius_rpc_https_url, commitment_config)
}
