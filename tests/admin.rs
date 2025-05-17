use vending_machines_nostr::admin::{self, setup_admin_handler};
use vending_machines_nostr::VendingMachine;
use vending_machines_nostr::admin::commands::{AddItemStruct, AdminCommand};
use tokio::sync::mpsc;

use nostr_sdk::{Client, Keys, Filter, EventBuilder, Tag};
use vending_machines_nostr::{
    Item,
};
use std::time::Duration;

#[tokio::test]
async fn test_status() {
    // Create a channel for admin commands
    let (tx, rx) = mpsc::channel(10);

    // Initialize the vending machine with the receiver
    let mut vending_machine = VendingMachine::new(rx);

    // Send an AddItem command
    tx.send(AdminCommand::Status).await.unwrap();

    // Process the command
    let result = vending_machine.process_next_admin_command().await;

    // Assert that the command was processed successfully
    assert_eq!(result.unwrap(), true);
}

#[tokio::test]
async fn test_status_command_via_nostr() {
    // Set up Nostr client
    let keys = Keys::generate();
    let admin_keys = Keys::generate();
    let client = nostr_sdk::ClientBuilder::new().signer(admin_keys.clone()).build();
    let relay_url = "wss://relay.damus.io"; // Local test relay
    client.add_relay(relay_url).await.unwrap();
    client.connect().await;

    // Set up vending machine with test items
    let (tx, rx) = mpsc::channel(10);
    // Create and configure admin handler
    let admin_handler = setup_admin_handler(keys.clone(), &[admin_keys.public_key().to_string()], tx.clone())
        .await
        .unwrap();

    // Create vending machine
    let mut vm = VendingMachine::new(rx);

    // Spawn admin listener task
    let _ = tokio::spawn(async move {
        if let Err(e) = admin_handler.handle_events().await {
            eprintln!("Admin handler error: {}", e);
        }
    });

    
    // Add test items to vending machine
    let test_item = Item::new(1, "Test Item".to_string(), 100, 5);
    vm.with_admin_privileges(|vm| vm.add_item(test_item)).unwrap();

    // Create and send Status command via Nostr
    let command = AdminCommand::Status;
    let command_json = serde_json::to_string(&command).unwrap();
    println!("command_json: {}", command_json);


    let encrypted = nostr_sdk::nips::nip44::encrypt(
        &admin_keys.secret_key(),
        &keys.public_key(),
        command_json,
        nostr_sdk::nips::nip44::Version::V2,
    ).unwrap();
    
    // Create event using correct builder method
    let event = EventBuilder::new(
        nostr_sdk::Kind::EncryptedDirectMessage,
        encrypted,
    ).build(admin_keys.public_key()).sign(&admin_keys).await.unwrap();

    println!("sending event");
    client.send_event(&event).await.unwrap();
    println!("sent event");

    // Clean up
    vm.process_next_admin_command().await.unwrap();
    client.disconnect().await;
}