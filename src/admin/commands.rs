use std::str::FromStr;

use serde::{Deserialize, Serialize};

use super::AdminError;

pub struct CommandTransport {
    id: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddItemStruct {
    pub id: u64,
    pub count: u64,
}

/// AdminCommand represents a command that can be sent by the admin via Nostr.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AdminCommand {
    /// Request the machine to reboot.
    Reboot,
    /// Request the machine to report its current status.
    Status,
    /// add Item
    AddItem(AddItemStruct),
}
