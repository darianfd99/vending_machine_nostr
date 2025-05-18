use std::time::Duration;

use tokio::sync::mpsc;
use vending_machines_nostr::admin::commands::{AddItemRequest, AdminCommand, ChangePriceRequest};
use vending_machines_nostr::admin::setup_admin_handler;
use vending_machines_nostr::vm::vending_machine::VendingMachine;

use nostr_sdk::{EventBuilder, Keys};
use vending_machines_nostr::vending_machine::Item;

#[tokio::test]
async fn test_add_item_command_via_nostr() {
    // Set up Nostr client
    let keys = Keys::generate();
    let admin_keys = Keys::generate();
    let client = nostr_sdk::ClientBuilder::new()
        .signer(admin_keys.clone())
        .build();
    let relay_url = "wss://relay.damus.io"; // Local test relay
    client.add_relay(relay_url).await.unwrap();
    client.connect().await;

    // Set up vending machine with test items
    let (tx, rx) = mpsc::channel(10);
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel::<bool>(1);
    // Create and configure admin handler
    let admin_handler = setup_admin_handler(
        keys.clone(),
        &[admin_keys.public_key().to_string()],
        tx.clone(),
    )
    .await
    .unwrap();

    // Create vending machine
    let mut vm = VendingMachine::new(rx, shutdown_rx);
    vm.admin().unwrap();
    let machine = tokio::spawn(async move {
        if let Err(e) = vm.run_machine().await {
            eprintln!("Vending machine error: {}", e);
        }

        let added_item = vm.get_item(42);
        assert!(added_item.is_some(), "Item should be added to the machine");

        if let Some(item) = added_item {
            assert_eq!(item.name, "Test Product");
            assert_eq!(item.price, 100);
            assert_eq!(item.count, 37);
        }
        vm.cancel().unwrap();
    });

    // Spawn admin listener task
    let _ = tokio::spawn(async move {
        if let Err(e) = admin_handler.handle_events().await {
            eprintln!("Admin handler error: {}", e);
        }
    });

    // Create AddItem command
    let add_item_data = AddItemRequest {
        id: 42,
        name: "Test Product".to_string(),
        price: 100,
        count: 5,
    };
    let command = AdminCommand::AddItem(add_item_data);
    let command_json = serde_json::to_string(&command).unwrap();

    // Encrypt command
    let encrypted = nostr_sdk::nips::nip44::encrypt(
        &admin_keys.secret_key(),
        &keys.public_key(),
        command_json,
        nostr_sdk::nips::nip44::Version::V2,
    )
    .unwrap();
    
    // Create and send event
    let event = EventBuilder::new(nostr_sdk::Kind::EncryptedDirectMessage, encrypted)
        .build(admin_keys.public_key())
        .sign(&admin_keys)
        .await
        .unwrap();

    println!("Sending AddItem command...");
    client.send_event(&event).await.unwrap();

    // Create AddItem command
    let add_item_data = AddItemRequest {
        id: 42,
        name: "Test Product".to_string(),
        price: 100,
        count: 32,
    };
    let command = AdminCommand::AddItem(add_item_data);
    let command_json = serde_json::to_string(&command).unwrap();

    // Encrypt command
    let encrypted = nostr_sdk::nips::nip44::encrypt(
        &admin_keys.secret_key(),
        &keys.public_key(),
        command_json,
        nostr_sdk::nips::nip44::Version::V2,
    )
    .unwrap();

    // Create and send event
    let event = EventBuilder::new(nostr_sdk::Kind::EncryptedDirectMessage, encrypted)
        .build(admin_keys.public_key())
        .sign(&admin_keys)
        .await
        .unwrap();

    println!("Sending AddItem command...");
    client.send_event(&event).await.unwrap();

    // Wait a bit for command processing
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Clean up
    shutdown_tx.send(true).await.unwrap();
    machine.await.unwrap();
    client.disconnect().await;
}

#[tokio::test]
async fn test_change_price_command_via_nostr() {
    // Set up Nostr client
    let keys = Keys::generate();
    let admin_keys = Keys::generate();
    let client = nostr_sdk::ClientBuilder::new()
        .signer(admin_keys.clone())
        .build();
    let relay_url = "wss://relay.damus.io";
    client.add_relay(relay_url).await.unwrap();
    client.connect().await;

    // Set up channels
    let (tx, rx) = mpsc::channel(10);
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel::<bool>(1);

    // Create admin handler
    let admin_handler = setup_admin_handler(
        keys.clone(),
        &[admin_keys.public_key().to_string()],
        tx.clone(),
    )
    .await
    .unwrap();

    // Create vending machine and add initial item
    let mut vm = VendingMachine::new(rx, shutdown_rx);
    vm.admin().unwrap();
    vm.add_item(Item::new(22, "Test Product".to_string(), 100, 5)).unwrap();
    assert!(vm.get_item(22).is_some());
    assert_eq!(vm.get_item(22).unwrap().price, 100);

    // Spawn machine task
    let machine = tokio::spawn(async move {
        if let Err(e) = vm.run_machine().await {
            eprintln!("Vending machine error: {}", e);
        }

        let item = vm.get_item(22).unwrap();
        assert_eq!(item.price, 150);
    });

    // Spawn admin handler
    let _ = tokio::spawn(async move {
        if let Err(e) = admin_handler.handle_events().await {
            eprintln!("Admin handler error: {}", e);
        }
    });

    // Create ChangePriceRequest command
    let change_price_req = ChangePriceRequest {
        id: 22,
        price: 150,
    };
    let command = AdminCommand::ChangePrice(change_price_req);
    let command_json = serde_json::to_string(&command).unwrap();

    // Send command via Nostr
    let encrypted = nostr_sdk::nips::nip44::encrypt(
        &admin_keys.secret_key(),
        &keys.public_key(),
        command_json,
        nostr_sdk::nips::nip44::Version::V2,
    )
    .unwrap();

    let event = EventBuilder::new(nostr_sdk::Kind::EncryptedDirectMessage, encrypted)
        .build(admin_keys.public_key())
        .sign(&admin_keys)
        .await
        .unwrap();

    println!("Sending ChangePrice command...");
    client.send_event(&event).await.unwrap();

    // Wait for command processing
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Clean up
    shutdown_tx.send(true).await.unwrap();
    machine.await.unwrap();
    client.disconnect().await;
}

#[tokio::test]
async fn test_remove_item_command_via_nostr() {
    // Set up Nostr client
    let keys = Keys::generate();
    let admin_keys = Keys::generate();
    let client = nostr_sdk::ClientBuilder::new()
        .signer(admin_keys.clone())
        .build();
    let relay_url = "wss://relay.damus.io";
    client.add_relay(relay_url).await.unwrap();
    client.connect().await;

    // Set up channels
    let (tx, rx) = mpsc::channel(10);
    let (shutdown_tx, shutdown_rx) = tokio::sync::mpsc::channel::<bool>(1);

    // Create admin handler
    let admin_handler = setup_admin_handler(
        keys.clone(),
        &[admin_keys.public_key().to_string()],
        tx.clone(),
    )
    .await
    .unwrap();

    // Create vending machine and add initial item
    let mut vm = VendingMachine::new(rx, shutdown_rx);
    vm.admin().unwrap();
    vm.add_item(Item::new(12, "Test Product".to_string(),34, 4)).unwrap();
    assert!(vm.get_item(12).is_some());

    // Spawn machine task
    let machine = tokio::spawn(async move {
        if let Err(e) = vm.run_machine().await {
            eprintln!("Vending machine error: {}", e);
        }

        let item = vm.get_item(12);
        assert!(item.is_none());
    });

    // Spawn admin handler
    let _ = tokio::spawn(async move {
        if let Err(e) = admin_handler.handle_events().await {
            eprintln!("Admin handler error: {}", e);
        }
    });

    // Create RemoveItem command
    let command = AdminCommand::RemoveItem(12);
    let command_json = serde_json::to_string(&command).unwrap();

    // Send command via Nostr
    let encrypted = nostr_sdk::nips::nip44::encrypt(
        &admin_keys.secret_key(),
        &keys.public_key(),
        command_json,
        nostr_sdk::nips::nip44::Version::V2,
    )
    .unwrap();

    let event = EventBuilder::new(nostr_sdk::Kind::EncryptedDirectMessage, encrypted)
        .build(admin_keys.public_key())
        .sign(&admin_keys)
        .await
        .unwrap();

    println!("Sending RemoveItem command...");
    client.send_event(&event).await.unwrap();

    // Wait for command processing
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Clean up
    shutdown_tx.send(true).await.unwrap();
    machine.await.unwrap();
    client.disconnect().await;
}