use dotenv::dotenv;
use solana_cookbook_rust::jwest_development::{
    connect_to_environment::create_rpc_client, get_test_sol::get_test_sol,
};
use solana_cookbook_rust::utils::Logger;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;

fn main() {
    let logger = Logger::new("[CookBook]".to_string());
    dotenv().ok();

    let commitment_config = CommitmentConfig::processed();
    let helius_rpc_url = env::var("HELIUS_RPC_URL").expect("HELIUS_RPC_URL not set");

    let client = create_rpc_client(helius_rpc_url, commitment_config);
    logger.log(format!(
        "Solana RPC Connection is healthy: {}",
        client.get_health().is_ok()
    ));

    match get_test_sol(client) {
        Ok(signature) => {
            logger.log(format!("Airdrop Signature is: {}", signature));
        }
        Err(e) => {
            logger.log(format!("{}", e));
        }
    }
}
