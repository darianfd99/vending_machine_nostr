use nostr_sdk::Keys;
use vending_machines_nostr::{
    admin::{commands::AdminCommand, setup_admin_handler},
    vending_machine::{VendingMachine, VendingMachineError},
};

#[tokio::main]
async fn main() -> Result<(), VendingMachineError> {
    // Create admin command channel
    let (tx, rx) = tokio::sync::mpsc::channel::<AdminCommand>(10);
    let (_, shutdown_rx) = tokio::sync::mpsc::channel::<bool>(1);

    let relays = &["ws://localhost:7777"];

    let admin_keys = Keys::generate();
    println!("Admin vm: {}", admin_keys.public_key());
    // Create and configure admin handler
    let admin_handler = setup_admin_handler(
        admin_keys.clone(),
        &["npub1agsuqc2g2slv3fnlf8xancqvzyywrwdf7sq4llhzuv48nz3evtcq555fmx".to_string()],
        relays,
        tx,
    )
    .await
    .map_err(VendingMachineError::AdminError)?;

    // Create vending machine
    let mut vm = VendingMachine::new(admin_keys, relays, rx, shutdown_rx).await?;

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
