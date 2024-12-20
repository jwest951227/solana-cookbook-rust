use solana_cookbook_rust::jwest_development::connect_to_environment;
use solana_cookbook_rust::utils::Logger;

fn main() {
    let client = connect_to_environment::create_rpc_client();
    let logger = Logger::new("[CookBook]".to_string());

    logger.log(format!(
        "Solana RPC Connection is healthy: {}",
        client.get_health().is_ok()
    ));
}
