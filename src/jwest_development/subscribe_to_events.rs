use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_pubkey::Pubkey;
use tokio_stream::StreamExt;

pub async fn subscribe_to_events(helius_rpc_wss_url: &str, account: &Pubkey) {
    let listener_client = match PubsubClient::new(helius_rpc_wss_url).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to create PubsubClient: {}", e);
            return;
        }
    };

    let (mut stream, shutdown_handle) = listener_client
        .account_subscribe(account, None)
        .await
        .unwrap();
    while let Some(data) = stream.next().await {
        println!("Received an update! {:?}", data);
    }
    shutdown_handle().await;
}
