use nostr_sdk::Keys;
use vending_machines_nostr::{
    admin::{builder::AdminHandlerBuilder, commands::AdminCommand, AdminError, AdminHandler},
    VendingMachine, VendingMachineError,
};

#[tokio::main]
async fn main() -> Result<(), VendingMachineError> {
    // Create admin command channel
    let (tx, rx) = tokio::sync::mpsc::channel::<AdminCommand>(10);

    // Create and configure admin handler
    let admin_handler = setup_admin_handler(tx.clone())
        .await
        .map_err(VendingMachineError::AdminError)?;

    // Create vending machine
    let mut vm = VendingMachine::new(rx);

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

async fn setup_admin_handler(
    sender: tokio::sync::mpsc::Sender<AdminCommand>,
) -> Result<AdminHandler, AdminError> {
    // Create client
    // Generate new random keys
    let keys = Keys::generate();
    let nostr_client = nostr_sdk::ClientBuilder::new().signer(keys.clone()).build();

    // Connect to relays
    nostr_client
        .add_relay("wss://relay.damus.io")
        .await
        .unwrap();

    nostr_client
        .add_read_relay("wss://relay.nostr.info")
        .await
        .unwrap();

    nostr_client.connect().await;

    // Build the admin handler
    let admin_handler = AdminHandlerBuilder::new()
        .client(nostr_client)
        .add_admin_pubkey("npub1sptdzjaupr677unjdad6s6vqjcry2sw8z3cz8tvy4slyvxltlv8sdgwdpl")?
        .private_key(keys.secret_key().clone())
        .sender_admin_commands(sender)
        .build()?;

    // Subscribe to events
    admin_handler.subscribe().await;

    Ok(admin_handler)
}
