use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AddItemRequest {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub count: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangePriceRequest {
    pub id: u64,
    pub price: u64,
}

/// AdminCommand represents a command that can be sent by the admin via Nostr.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum AdminCommand {
    /// Request Admin State
    RequestAdminState,
    /// Request the machine to reboot.
    Reboot,
    /// Request the machine to report its current status.
    Status,
    /// add Item
    AddItem(AddItemRequest),
    /// Remove item
    RemoveItem(u64),
    /// Change price
    ChangePrice(ChangePriceRequest),
}
