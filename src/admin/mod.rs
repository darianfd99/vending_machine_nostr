pub mod builder;
pub mod commands;
mod helper;

use builder::AdminHandlerBuilder;
use commands::AdminCommand;
use nostr_sdk::Client;
use std::collections::HashSet;
use tokio::sync::mpsc;

/// Enum representing errors related to admin handling.
#[derive(Debug)]
pub enum AdminError {
    /// An unknown command was received.
    UnknownCommand(String),

    /// The Nostr public key provided for an admin is invalid.
    InvalidNostrPubKey(String),

    /// Missing Nostr client when attempting to build the AdminHandler
    MissingClient(String),

    /// No admin public keys provided when attempting to build the AdminHandler
    MissingAdminPubKeys(String),

    /// No private key provided
    MissingPrivateKey(String),

    /// Shutdown error
    ShutdownError(String),

    /// Handle notifications error
    HandleNotifications(String),

    /// Relay error
    Relay(String),
}

/// Represents the handler for processing and managing admin-related Nostr events and commands.
///
/// This struct holds the Nostr `Client` instance for interacting with the Nostr network and a set
/// of authorized admin public keys (either as hex-encoded strings or Bech32 `npub1...` keys).
///
/// It is responsible for ensuring that only authorized admins can issue commands.
pub struct AdminHandler {
    /// The Nostr client used for network interactions
    client: Client,

    /// Set of authorized Nostr public keys for admins
    admin_pubkeys: HashSet<nostr_sdk::PublicKey>,

    /// nostr private key
    key: nostr_sdk::SecretKey,

    /// command producer
    send_admin_commands: mpsc::Sender<AdminCommand>,
}

/// Builder for the `AdminHandler` struct, allowing flexible and validated construction.
///
/// The builder pattern allows incremental construction of an `AdminHandler` by first setting
/// the `Client` and then adding admin public keys. Once the required fields are set, the handler can
/// be constructed using the `.build()` method.
impl AdminHandler {
    /// Subscribes the handler to listen for commands from the admin.
    pub async fn subscribe(&self) {
        let filter = nostr_sdk::Filter::new()
            .kinds(vec![nostr_sdk::Kind::EncryptedDirectMessage])
            .authors(self.admin_pubkeys.clone());

        let _ = self.client.subscribe(filter, None).await;
    }

    pub async fn handle_events(&self) -> Result<(), Box<dyn std::error::Error>> {
        let admin_pubkeys = &self.admin_pubkeys;
        let client = &self.client;

        client
            .handle_notifications(|notification| async move {
                if let nostr_sdk::RelayPoolNotification::Event {
                    event,
                    subscription_id: _,
                    ..
                } = notification
                {
                    if admin_pubkeys.contains(&event.pubkey)
                        && event.kind == nostr_sdk::Kind::EncryptedDirectMessage
                    {
                        // Attempt to decrypt using NIP-44
                        if let Ok(decrypted_command) = nostr_sdk::nips::nip44::decrypt(
                            &self.key,
                            &event.pubkey,
                            &event.content,
                        ) {
                            println!("🔐 Decrypted NIP-44 message: {}", decrypted_command);
                            if let Ok(command) =
                                serde_json::from_str::<AdminCommand>(&decrypted_command)
                            {
                                let _ = self.send_admin_commands.send(command.clone()).await;
                                if let AdminCommand::Shutdown = command {
                                    return Ok(true);
                                }
                            } else {
                                eprintln!("incorrect format for command");
                            }
                        } else {
                            eprintln!("error while decrypting");
                        }
                    }
                }
                Ok(false) // Keep listening
            })
            .await?;

        client.disconnect().await;
        client.shutdown().await;

        Ok(())
    }
}

pub async fn setup_admin_handler(
    keys: nostr_sdk::Keys,
    pubkeys: &[String],
    admin_relays: &[&str],
    sender: tokio::sync::mpsc::Sender<AdminCommand>,
) -> Result<AdminHandler, AdminError> {
    // Create client
    // Generate new random keys
    let nostr_client = nostr_sdk::ClientBuilder::new().signer(keys.clone()).build();

    // Connect to relays
    for &relay in admin_relays {
        nostr_client
            .add_relay(relay)
            .await
            .map_err(|_| AdminError::Relay(format!("Failed to add relay: {}", relay)))?;
    }
    nostr_client.connect().await;

    // Build the admin handler
    let mut admin_handler_builder = AdminHandlerBuilder::new()
        .client(nostr_client)
        .private_key(keys.secret_key().clone())
        .sender_admin_commands(sender);

    for pubkey in pubkeys {
        admin_handler_builder = admin_handler_builder.add_admin_pubkey(pubkey)?;
    }

    let admin_handler = admin_handler_builder.build()?;

    // Subscribe to events
    admin_handler.subscribe().await;

    Ok(admin_handler)
}
