use solana_client::nonblocking::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_sdk::hash::Hash;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::system_transaction;
use solana_signature::Signature;

pub async fn send_sol(
    rpc_client: RpcClient,
    from_keypair: &Keypair,
    to: &Pubkey,
    lamports: u64,
    latest_blockhash: Hash,
) -> Result<Signature, String> {
    let tx = system_transaction::transfer(from_keypair, to, lamports, latest_blockhash);
    rpc_client
        .send_and_confirm_transaction(&tx)
        .await
        .map_err(|e| format!("Transaction Failed: {}", e))
}
