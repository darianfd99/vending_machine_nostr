use std::collections::HashSet;

use tokio::sync::mpsc;

use super::{commands::AdminCommand, helper, AdminError, AdminHandler};

pub struct AdminHandlerBuilder {
    /// The Nostr client to be used with the handler
    client: Option<nostr_sdk::Client>,

    /// Set of admin public keys that will be authorized to issue commands
    admin_pubkeys: HashSet<nostr_sdk::PublicKey>,

    /// nostr private key
    key: Option<nostr_sdk::SecretKey>,

    /// admin commands sender
    admin_commands_sender: Option<mpsc::Sender<AdminCommand>>,
}

impl Default for AdminHandlerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl AdminHandlerBuilder {
    /// Creates a new empty `AdminHandlerBuilder` instance.
    ///
    /// This function initializes the builder with no `Client` and an empty set of admin public keys.
    /// It is intended for incrementally setting up the `AdminHandler` via subsequent method calls.
    ///
    /// # Returns
    /// A new instance of `AdminHandlerBuilder`.
    pub fn new() -> Self {
        Self {
            client: None,
            admin_pubkeys: HashSet::new(),
            key: None,
            admin_commands_sender: None,
        }
    }

    /// Sets the Nostr `Client` instance to be used for network interactions.
    ///
    /// The `Client` is necessary to perform interactions with the Nostr network, such as sending events.
    ///
    /// # Arguments
    ///
    /// * `client` - The `Client` instance to be used for network interactions.
    ///
    /// # Returns
    /// The builder instance with the `Client` set.
    pub fn client(mut self, client: nostr_sdk::Client) -> Self {
        self.client = Some(client);
        self
    }

    /// Adds an admin public key (either hex or Bech32) to the builder.
    ///
    /// This function attempts to parse the provided string as either a valid 64-character hex string
    /// or a Bech32-encoded Nostr public key. If the public key is valid, it is added to the set of
    /// authorized admin public keys. If the pubkey is invalid, an error is returned.
    ///
    /// # Arguments
    ///
    /// * `input` - The public key in either hex or Bech32 format.
    ///
    /// # Returns
    /// A result containing either:
    /// - `Ok(self)` if the public key was successfully added.
    /// - `Err(AdminError)` if the public key format is invalid.
    pub fn add_admin_pubkey<S: Into<String>>(mut self, input: S) -> Result<Self, AdminError> {
        let raw = input.into();

        if let Some(pk) = helper::parse_pubkey(&raw) {
            self.admin_pubkeys.insert(pk);
        } else {
            return Err(AdminError::InvalidNostrPubKey(format!(
                "⚠️ Invalid pubkey format: {}",
                raw
            )));
        }

        Ok(self)
    }

    pub fn private_key(mut self, key: nostr_sdk::SecretKey) -> Self {
        self.key = Some(key);
        self
    }

    pub fn sender_admin_commands(mut self, sender: mpsc::Sender<AdminCommand>) -> Self {
        self.admin_commands_sender = Some(sender);
        self
    }

    /// Builds the `AdminHandler` struct, ensuring all required fields are provided and valid.
    ///
    /// # Returns
    /// - `Ok(AdminHandler)` if both the `Client` and at least one valid admin public key are provided.
    /// - `Err(AdminError::MissingClient)` if no `Client` is provided.
    /// - `Err(AdminError::MissingAdminPubKeys)` if no valid admin public keys are provided.
    pub fn build(self) -> Result<AdminHandler, AdminError> {
        // Ensure that the client is set
        let client = self.client.ok_or_else(|| {
            AdminError::MissingClient(
                "Missing client. A Nostr client must be provided.".to_string(),
            )
        })?;

        // Ensure there is at least one admin public key
        if self.admin_pubkeys.is_empty() {
            return Err(AdminError::MissingAdminPubKeys(
                "No valid admin pubkeys provided.".to_string(),
            ));
        }

        // Ensure there is a private key
        let key = self.key.ok_or_else(|| {
            AdminError::MissingPrivateKey(
                "Mising private key. A private key must be provided for decryption".to_string(),
            )
        })?;

        // Ensure there is an admin commands sender
        let send_admin_commands = self.admin_commands_sender.ok_or_else(|| {
            AdminError::MissingPrivateKey(
                "Mising admin command sender. The sender must be provided".to_string(),
            )
        })?;

        // If validation passed, return the AdminHandler
        Ok(AdminHandler {
            client,
            admin_pubkeys: self.admin_pubkeys,
            key,
            send_admin_commands,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nostr_sdk::Client;

    #[test]
    fn test_builder_new() {
        let builder = AdminHandlerBuilder::new();
        assert!(builder.client.is_none());
        assert!(builder.admin_pubkeys.is_empty());
    }

    #[test]
    fn test_builder_client_assignment() {
        let client = Client::default();
        let builder = AdminHandlerBuilder::new().client(client);
        assert!(builder.client.is_some());
    }

    #[test]
    fn test_builder_add_admin_valid_pubkey() {
        let builder = AdminHandlerBuilder::new();
        let result = builder
            .add_admin_pubkey("8c2fa6ac7b9f09d8d5ad52be317bf1f8eab428f3ffb3c15e0420be9e97f0d387");
        assert!(result.is_ok());
        assert_eq!(result.unwrap().admin_pubkeys.len(), 1);
    }

    #[test]
    fn test_builder_add_admin_invalid_pubkey() {
        let builder = AdminHandlerBuilder::new();
        let result = builder.add_admin_pubkey("invalid_pubkey");
        assert!(matches!(result, Err(AdminError::InvalidNostrPubKey(_))));
    }

    #[test]
    fn test_build_missing_client_error() {
        let builder = AdminHandlerBuilder::new()
            .add_admin_pubkey("8c2fa6ac7b9f09d8d5ad52be317bf1f8eab428f3ffb3c15e0420be9e97f0d387")
            .unwrap();

        let result = builder.build();
        assert!(matches!(result, Err(AdminError::MissingClient(_))));
    }

    #[test]
    fn test_build_missing_pubkeys_error() {
        let client = Client::default();
        let builder = AdminHandlerBuilder::new().client(client);
        let result = builder.build();
        assert!(matches!(result, Err(AdminError::MissingAdminPubKeys(_))));
    }

    #[test]
    fn test_build_successful() {
        let (tx, _) = mpsc::channel(10);
        let client = Client::default();
        let builder = AdminHandlerBuilder::new()
            .client(client)
            .add_admin_pubkey("8c2fa6ac7b9f09d8d5ad52be317bf1f8eab428f3ffb3c15e0420be9e97f0d387")
            .unwrap()
            .private_key(nostr_sdk::SecretKey::generate())
            .sender_admin_commands(tx);

        let result = builder.build();
        assert!(result.is_ok());
        let handler = result.unwrap();
        assert_eq!(handler.admin_pubkeys.len(), 1);
    }
}
