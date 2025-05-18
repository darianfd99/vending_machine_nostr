use nostr_sdk::Keys;
use vending_machines_nostr::{
    admin::{
        builder::AdminHandlerBuilder, commands::AdminCommand, setup_admin_handler, AdminError,
        AdminHandler,
    },
    vending_machine::{VendingMachine, VendingMachineError},
};

#[tokio::main]
async fn main() -> Result<(), VendingMachineError> {
    // Create admin command channel
    let (tx, rx) = tokio::sync::mpsc::channel::<AdminCommand>(10);
    let (_, shutdown_rx) = tokio::sync::mpsc::channel::<bool>(1);

    // Create and configure admin handler
    let admin_handler = setup_admin_handler(Keys::generate(), &["".to_string()], tx.clone())
        .await
        .map_err(VendingMachineError::AdminError)?;

    // Create vending machine
    let mut vm = VendingMachine::new(rx, shutdown_rx);

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
