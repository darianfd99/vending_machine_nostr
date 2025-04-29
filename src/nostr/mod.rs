pub mod alert_system;

use alert_system::types::{AlertLevel, AlertPayload, AlertSource, AlertType};
use nostr_sdk::{Client, EventBuilder, Keys, Kind, Result};
use std::collections::HashMap;

pub struct AlertSystem {
    pub machine_id: String,
    client: Client,
}

impl AlertSystem {
    pub async fn new(machine_id: impl Into<String>, keys: Keys, relay: &str) -> Result<Self> {
        let client = Client::new(keys);
        client.add_relay(relay).await?;
        client.connect().await;

        Ok(Self {
            machine_id: machine_id.into(),
            client,
        })
    }

    pub async fn send_custom_alert(&self, payload: AlertPayload) -> Result<()> {
        let json = serde_json::to_string(&payload)?;
        let builder = EventBuilder::new(Kind::Custom(40001), json);
        self.client.send_event_builder(builder).await?;
        Ok(())
    }

    // -- High-level helpers --

    pub async fn send_low_inventory(&self, slot_id: u8, remaining: u32) -> Result<()> {
        let mut data = HashMap::new();
        data.insert("slot".into(), slot_id.into());
        data.insert("remaining".into(), remaining.into());

        let payload = AlertPayload {
            machine_id: self.machine_id.clone(),
            alert_type: AlertType::LowInventory,
            level: AlertLevel::Warning,
            source: AlertSource::Slot(slot_id),
            message: format!("Slot {} only has {} items left", slot_id, remaining),
            data,
        };

        self.send_custom_alert(payload).await
    }

    pub async fn send_jam_error(&self, motor_id: &str) -> Result<()> {
        let mut data = HashMap::new();
        data.insert("motor".into(), motor_id.into());

        let payload = AlertPayload {
            machine_id: self.machine_id.clone(),
            alert_type: AlertType::Jam,
            level: AlertLevel::Critical,
            source: AlertSource::Motor(motor_id.into()),
            message: format!("Motor '{}' is jammed", motor_id),
            data,
        };

        self.send_custom_alert(payload).await
    }

    pub async fn send_overheat_alert(&self, temperature: f32) -> Result<()> {
        let mut data = HashMap::new();
        data.insert("temperature".into(), temperature.into());

        let payload = AlertPayload {
            machine_id: self.machine_id.clone(),
            alert_type: AlertType::Overheat,
            level: AlertLevel::Critical,
            source: AlertSource::Network,
            message: format!("Machine is overheating. Current temp: {}°C", temperature),
            data,
        };

        self.send_custom_alert(payload).await
    }
}
