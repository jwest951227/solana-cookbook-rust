use solana_client::nonblocking::rpc_client::RpcClient;
use solana_pubkey::Pubkey;

pub async fn get_balance(rpc_client: RpcClient, account_pubkey: &Pubkey) -> u64 {
    rpc_client
        .get_account(account_pubkey)
        .await
        .unwrap()
        .lamports
}
