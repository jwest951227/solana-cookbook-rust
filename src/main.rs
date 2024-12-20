use dotenv::dotenv;
use solana_cookbook_rust::jwest_development::{
    connect_to_environment::create_rpc_client, get_test_sol::get_test_sol,
    subscribe_to_events::subscribe_to_events,
};
use solana_cookbook_rust::utils::Logger;
use solana_pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use std::env;

#[tokio::main]
async fn main() {
    let logger = Logger::new("[CookBook]".to_string());
    dotenv().ok();

    let commitment_config = CommitmentConfig::processed();
    let helius_rpc_https_url =
        env::var("HELIUS_RPC_HTTPS_URL").expect("HELIUS_RPC_HTTPS_URL not set");

    let client = create_rpc_client(helius_rpc_https_url.clone(), commitment_config);
    logger.log(format!(
        "Solana RPC Connection is healthy: {}",
        client.get_health().is_ok()
    ));

    match get_test_sol(client) {
        Ok(signature) => {
            logger.log(format!("Airdrop Signature is: {}", signature));
        }
        Err(e) => {
            logger.log(e.to_string());
        }
    }

    let helius_rpc_wss_url = env::var("HELIUS_RPC_WSS_URL").expect("HELIUS_RPC_WSS_URL not set");
    let wallet_account: Pubkey =
        Pubkey::from_str_const("BWnMWUuVc837EyFhiRt6cRwGcULfQLyf2qNbQJbG7g8R");
    subscribe_to_events(helius_rpc_wss_url.as_str(), &wallet_account).await;
}
