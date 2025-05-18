use std::time::Duration;
use tokio::time::sleep;

pub const LOCAL_RELAY_URL: &str = "ws://localhost:7777";

pub async fn setup_local_relay_client(keys: nostr_sdk::Keys) -> nostr_sdk::Client {
    let client = nostr_sdk::ClientBuilder::new().signer(keys).build();

    client.add_relay(LOCAL_RELAY_URL).await.unwrap();
    client.connect().await;

    // Wait for connection
    sleep(Duration::from_secs(1)).await;

    client
}
