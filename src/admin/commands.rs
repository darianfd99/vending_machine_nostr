use std::str::FromStr;

use serde::Deserialize;

use super::AdminError;

pub struct CommandTransport {
    id: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AddItemStruct {
    id: u64,
    count: u64,
}

/// AdminCommand represents a command that can be sent by the admin via Nostr.
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum AdminCommand {
    /// Request the machine to reboot.
    Reboot,
    /// Request the machine to report its current status.
    Status,
    /// add Item
    AddItem(AddItemStruct),
}

impl FromStr for AdminCommand {
    type Err = AdminError;

    /// Attempts to parse a command from a raw string.
    ///
    /// Returns an `AdminCommand`, matching known commands or failing.
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.trim().to_lowercase().as_str() {
            "reboot" => Ok(AdminCommand::Reboot),
            "status" => Ok(AdminCommand::Status),
            other => Err(AdminError::UnknownCommand(other.to_string())),
        }
    }
}
