use dotenv::dotenv;
use solana_cookbook_rust::utils::Logger;
use solana_cookbook_rust::{
    jwest_development::{
        create_nonblocking_rpc_client::create_nonblocking_rpc_client,
        create_rpc_client::create_rpc_client, get_test_sol::get_test_sol,
        subscribe_to_events::subscribe_to_events,
    },
    jwest_transactions::send_sol::send_sol,
    jwest_wallets::create_keypair::new,
};
use solana_pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};
use std::env;

#[tokio::main]
async fn main() {
    let logger = Logger::new("[CookBook]".to_string());
    dotenv().ok();

    /* Development Section */
    // run_development_section(&logger).await;

    /* Wallets Section */
    // _run_wallets_section(&logger);

    /* Transactions Section */
    run_transactions_section(&logger).await;
}

async fn _run_development_section(logger: &Logger) {
    let helius_rpc_https_url =
        env::var("HELIUS_RPC_HTTPS_URL").expect("HELIUS_RPC_HTTPS_URL not set");

    let client = create_rpc_client(helius_rpc_https_url.clone(), CommitmentConfig::processed());
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

fn _run_wallets_section(logger: &Logger) {
    let keypair = new();
    logger.log(format!("New Key Pair: {:?}", keypair));
    logger.log(format!(
        "New Key Pair in Base58 String(PrivateKey): {}",
        keypair.to_base58_string()
    ));
    logger.log(format!("New Key Pair's SecretKey: {:?}", keypair.secret()));
    logger.log(format!("New Key Pair's PublicKey: {:?}", keypair.pubkey()));
}

async fn run_transactions_section(logger: &Logger) {
    let helius_dev_rpc_https_url =
        env::var("HELIUS_DEVNET_RPC_HTTPS_URL").expect("HELIUS_DEVNET_RPC_HTTPS_URL not set");

    let client = create_nonblocking_rpc_client(
        helius_dev_rpc_https_url.clone(),
        CommitmentConfig::processed(),
    );
    logger.log(format!(
        "Solana Devnet RPC Connection is healthy: {}",
        client.get_health().await.is_ok()
    ));

    let from_keypair: Keypair = Keypair::from_base58_string(
        "JZjXrsKPVsFmgezJF6W3qyNmYkY7NzEAmAw1EWVkpAZV56haKwP7wvqzi4WocQK5NNs49XfHJQsfqqN29jDg3Tr",
    );
    let to: Pubkey = Pubkey::from_str_const("BUQWwJsWgScreFuMEgMFThYECRUz5xRoyLkn7qd4TB59");
    let lamports: u64 = 1000000000;
    let latest_blockhash = client.get_latest_blockhash().await.unwrap();
    match send_sol(client, &from_keypair, &to, lamports, latest_blockhash).await {
        Ok(signature) => {
            logger.log(format!("Sending Sol Transaction is: {}", signature));
        }
        Err(e) => {
            logger.log(e.to_string());
        }
    }
}
