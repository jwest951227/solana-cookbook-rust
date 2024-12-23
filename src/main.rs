use dotenv::dotenv;
use solana_cookbook_rust::utils::Logger;
use solana_cookbook_rust::{
    jwest_development::{
        create_nonblocking_rpc_client::create_nonblocking_rpc_client,
        create_rpc_client::create_rpc_client, get_test_sol::get_test_sol,
        subscribe_to_events::subscribe_to_events,
    },
    jwest_transactions::{add_priority_fees::add_priority_fees, send_sol::send_sol},
    jwest_wallets::create_keypair::new,
};
use solana_pubkey::Pubkey;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_sdk::signature::{Keypair, Signer};
use solana_transaction_status_client_types::UiTransactionEncoding;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let logger = Logger::new("[CookBook]".to_string());
    dotenv().ok();

    /* Development Section */
    // run_development_section(&logger).await;

    /* Wallets Section */
    // run_wallets_section(&logger);

    /* Sol Send Transactions Section */
    // _run_sol_send_tx_section(&logger).await;

    /* Sol Send Transactions Section */
    run_add_priority_fee_section(&logger).await;
}

async fn _run_development_section(logger: &Logger) {
    let rpc_https_url = env::var("RPC_HTTPS_URL").expect("RPC_HTTPS_URL not set");

    let client = create_rpc_client(rpc_https_url.clone(), CommitmentConfig::processed());
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

    let rpc_wss_url = env::var("RPC_WSS_URL").expect("RPC_WSS_URL not set");
    let wallet_account: Pubkey =
        Pubkey::from_str_const("BWnMWUuVc837EyFhiRt6cRwGcULfQLyf2qNbQJbG7g8R");
    subscribe_to_events(rpc_wss_url.as_str(), &wallet_account).await;
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

async fn _run_sol_send_tx_section(logger: &Logger) {
    let dev_rpc_https_url = env::var("DEVNET_RPC_HTTPS_URL").expect("DEVNET_RPC_HTTPS_URL not set");

    let client =
        create_nonblocking_rpc_client(dev_rpc_https_url.clone(), CommitmentConfig::processed());
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

async fn run_add_priority_fee_section(logger: &Logger) {
    let dev_rpc_https_url = env::var("DEVNET_RPC_HTTPS_URL").expect("DEVNET_RPC_HTTPS_URL not set");

    let client =
        create_nonblocking_rpc_client(dev_rpc_https_url.clone(), CommitmentConfig::processed());
    logger.log(format!(
        "Solana Devnet RPC Connection is healthy: {}",
        client.get_health().await.is_ok()
    ));

    let from_keypair: Keypair = Keypair::from_base58_string(
        "JZjXrsKPVsFmgezJF6W3qyNmYkY7NzEAmAw1EWVkpAZV56haKwP7wvqzi4WocQK5NNs49XfHJQsfqqN29jDg3Tr",
    );
    let to: Pubkey = Pubkey::from_str_const("BUQWwJsWgScreFuMEgMFThYECRUz5xRoyLkn7qd4TB59");
    let lamports: u64 = 1000000000;
    let latest_blockhash = match client.get_latest_blockhash().await {
        Ok(latest_blockhash) => latest_blockhash,
        Err(e) => {
            eprintln!("Failed to get the latest blockhash: {:?}", e);
            return;
        }
    };

    let client = Arc::new(client);
    let client_ref = client.as_ref();
    match add_priority_fees(
        client_ref,
        &from_keypair,
        &to,
        lamports,
        latest_blockhash,
        1000000,
        1,
    )
    .await
    {
        Ok(signature) => {
            logger.log(format!("Sending Sol Signature is: {}", signature));
            match client
                .get_transaction(&signature, UiTransactionEncoding::JsonParsed)
                .await
            {
                Ok(tx) => {
                    logger.log(format!("Sending Sol Transaction is: {:?}", tx.transaction));
                }
                Err(e) => {
                    logger.log(e.to_string());
                }
            }
        }
        Err(e) => {
            logger.log(e.to_string());
        }
    }
}
