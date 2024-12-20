use solana_client::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_sdk::native_token::LAMPORTS_PER_SOL;
use solana_signature::Signature;

pub fn get_test_sol(client: RpcClient) -> Result<Signature, String> {
    client
        .request_airdrop(
            &Pubkey::from_str_const("BUQWwJsWgScreFuMEgMFThYECRUz5xRoyLkn7qd4TB59"),
            LAMPORTS_PER_SOL,
        )
        .map_err(|e| format!("Airdrop failed: {}", e))
}
