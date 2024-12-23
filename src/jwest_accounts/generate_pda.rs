use solana_pubkey::Pubkey;
use solana_sdk::{bs58, system_program};

const PROGRAM_ID: Pubkey = system_program::id();

pub fn generate_pda() {
    let seeds = &["test".as_bytes()];
    let (pda, bump) = Pubkey::find_program_address(seeds, &PROGRAM_ID);

    println!(
        "pubkey: {}, bump: {}",
        bs58::encode(pda.to_bytes()).into_string(),
        bump
    )
}
