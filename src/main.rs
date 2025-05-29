use nostr_sdk::Keys;
use serde::Deserialize;
use std::fs;
use vending_machines_nostr::{
    admin::{commands::AdminCommand, setup_admin_handler},
    vending_machine::{VendingMachine, VendingMachineError},
};

#[derive(Deserialize)]
struct Config {
    admins: AdminConfig,
    relays: RelayConfig,
}

#[derive(Deserialize)]
struct AdminConfig {
    public_keys: Vec<String>,
}

#[derive(Deserialize)]
struct RelayConfig {
    addresses: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), VendingMachineError> {
    // Load configuration
    let config: Config = toml::from_str(
        &fs::read_to_string("config.toml")
            .map_err(|e| VendingMachineError::Config(e.to_string()))?
    ).map_err(|e| VendingMachineError::Config(e.to_string()))?;

    let relay_addresses: Vec<&str> = config.relays.addresses
        .iter()
        .map(AsRef::as_ref)
        .collect();

    // Create admin command channel
    let (tx, rx) = tokio::sync::mpsc::channel::<AdminCommand>(10);
    let (_, shutdown_rx) = tokio::sync::mpsc::channel::<bool>(1);

    let admin_keys = Keys::generate();
    println!("Admin vm: {}", admin_keys.public_key());
    
    // Create and configure admin handler with config
    let admin_handler = setup_admin_handler(
        admin_keys.clone(),
        &config.admins.public_keys,
        &relay_addresses,
        tx,
    )
    .await
    .map_err(VendingMachineError::AdminError)?;

    // Create vending machine
    let mut vm = VendingMachine::new(admin_keys, &relay_addresses, rx, shutdown_rx).await?;

    // Spawn admin listener task
    let admin_task = tokio::spawn(async move {
        if let Err(e) = admin_handler.handle_events().await {
            eprintln!("Admin handler error: {}", e);
        }
    });

    // Run the main machine loop
    vm.run_machine().await?;

    // Clean shutdown (optional)
    admin_task.abort();

    Ok(())
}
