use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::sysvar::rent::Rent;
use solana_sdk::{hash::Hash, signature::Keypair, system_program, system_transaction};
use solana_signature::Signature;

pub async fn create_account(
    rpc_client: &RpcClient,
    from_keypair: &Keypair,
    new_account: &Keypair,
    space: usize,
    latest_blockhash: Hash,
) -> Result<Signature, String> {
    let rent = Rent::default();
    let rent_exeption_amount = rent.minimum_balance(space);

    let token_mint_a_account_tx = system_transaction::create_account(
        from_keypair,
        new_account,
        latest_blockhash,
        rent_exeption_amount,
        space as u64,
        &system_program::id(),
    );

    rpc_client
        .send_and_confirm_transaction(&token_mint_a_account_tx)
        .await
        .map_err(|e| format!("Transaction Failed: {}", e))
}
