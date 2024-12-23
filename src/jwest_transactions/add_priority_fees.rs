use solana_client::nonblocking::rpc_client::RpcClient;
use solana_pubkey::Pubkey;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    hash::Hash,
    instruction::Instruction,
    signature::{Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use solana_signature::Signature;

pub async fn add_priority_fees(
    rpc_client: &RpcClient,
    from_keypair: &Keypair,
    to: &Pubkey,
    lamports: u64,
    latest_blockhash: Hash,
    units: u32,
    micro_lamports: u64,
) -> Result<Signature, String> {
    let modify_compute_units = ComputeBudgetInstruction::set_compute_unit_limit(units);
    let add_priority_fee = ComputeBudgetInstruction::set_compute_unit_price(micro_lamports);
    let mut instructions: Vec<Instruction> = Vec::new();
    instructions.insert(0, modify_compute_units);
    instructions.insert(1, add_priority_fee);

    let instruction = system_instruction::transfer(&from_keypair.pubkey(), to, lamports);
    instructions.insert(2, instruction);

    let txn = Transaction::new_signed_with_payer(
        &instructions,
        Some(&from_keypair.pubkey()),
        &vec![from_keypair],
        latest_blockhash,
    );

    rpc_client
        .send_and_confirm_transaction(&txn)
        .await
        .map_err(|e| format!("Transaction Failed: {}", e))
}
