use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertType {
    LowInventory,
    Jam,
    Overheat,
    Maintenance,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertLevel {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AlertSource {
    Motor(String),
    Slot(u8),
    Network,
}

#[derive(Debug, Serialize)]
pub struct AlertPayload {
    pub machine_id: String,
    pub alert_type: AlertType,
    pub level: AlertLevel,
    pub source: AlertSource,
    pub message: String,
    pub data: HashMap<String, serde_json::Value>,
}
