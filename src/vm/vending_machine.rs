use std::{collections::HashMap, fmt::Display};

use serde::Serialize;
use tokio::{sync::mpsc, time::Instant};

use super::{helper, listening_state::ListeningState, state::State};
use crate::admin::{commands::AdminCommand, AdminError};

#[derive(Debug)]
pub enum VendingMachineError {
    OutOfStock(&'static str),
    AddItem(&'static str),
    Dispense(&'static str),
    InsertMoney(&'static str),
    RequestItem(&'static str),
    Unauthorized(&'static str),
    AdminError(AdminError),
    ItemDoesNotExist(u64),
    Nostr(nostr_sdk::client::Error),
}

impl Display for VendingMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfStock(s) => write!(f, "VendingMachineError::OutOfStock: {}", s),
            Self::Dispense(s) => write!(f, "VendingMachineError::DispenseWithoutSelection: {}", s),
            Self::InsertMoney(s) => write!(f, "VendingMachineError::InsertMoney: {}", s),
            Self::AddItem(s) => write!(f, "VendingMachineError::AddItem: {}", s),
            Self::RequestItem(s) => write!(f, "VendingMachineError::RequestItem: {}", s),
            Self::Unauthorized(s) => write!(f, "VendingMachineError::Unauthorized: {}", s),
            Self::AdminError(s) => write!(f, "VendingMachineError::AdminError: {:?}", s),
            Self::ItemDoesNotExist(s) => {
                write!(f, "VendingMachineError::ItemDoesNotExist: {:?}", s)
            }
            Self::Nostr(s) => write!(f, "VendingMachineError::Nostr: {:?}", s),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Item {
    pub id: u64,
    pub name: String,
    pub price: u64,
    pub count: u64,
}

impl Item {
    pub fn new(id: u64, name: String, price: u64, count: u64) -> Self {
        Self {
            id,
            name,
            price,
            count,
        }
    }

    pub(crate) fn increment_count(&mut self, count: u64) {
        self.count += count;
    }

    pub(crate) fn sell_unit(&mut self) {
        self.count -= 1;
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct VendingMachineUpdate {
    pub under_admin: bool,
    pub items: Vec<Item>,
    pub state: String,
}

pub struct VendingMachine {
    pub(crate) under_admin: bool,
    state: Option<Box<dyn State>>,
    items: HashMap<u64, Item>,
    admin_commands: mpsc::Receiver<AdminCommand>,
    nostr_client: nostr_sdk::Client,
    shutdown: mpsc::Receiver<bool>,
    last_activity: Option<Instant>,
}

impl VendingMachine {
    pub async fn new(
        nostr_keys: nostr_sdk::Keys,
        admin_relays: &[&str],
        admin_commands: mpsc::Receiver<AdminCommand>,
        shutdown: mpsc::Receiver<bool>,
    ) -> Result<Self, VendingMachineError> {
        let nostr_client = nostr_sdk::ClientBuilder::new()
            .signer(nostr_keys.clone())
            .build();

        // Connect to relays
        for &relay in admin_relays {
            nostr_client
                .add_relay(relay)
                .await
                .map_err(VendingMachineError::Nostr)?;
        }
        nostr_client.connect().await;

        Ok(Self {
            under_admin: false,
            state: Some(Box::new(ListeningState)),
            items: HashMap::new(),
            admin_commands,
            last_activity: None,
            shutdown,
            nostr_client,
        })
    }

    pub fn is_under_admin(&self) -> bool {
        self.under_admin
    }

    pub async fn send_update(&self) -> Result<(), VendingMachineError> {
        let state_name = match self.state.as_ref() {
            Some(state) => {
                // Get concrete type name using std::any::type_name
                let type_name = std::any::type_name_of_val(&**state);
                // Extract just the struct name from the fully qualified path
                type_name
                    .split("::")
                    .last()
                    .unwrap_or("Unknown")
                    .to_string()
            }
            None => "NoState".to_string(),
        };

        let update = VendingMachineUpdate {
            under_admin: self.under_admin,
            items: self.items.values().cloned().collect(),
            state: state_name,
        };

        // Send the update to the Nostr client
        let event_builder = nostr_sdk::EventBuilder::new(
            nostr_sdk::Kind::TextNote,
            serde_json::to_string(&update).unwrap(),
        )
        .tag(nostr_sdk::Tag::all_relays())
        .tag(nostr_sdk::Tag::identifier("vending_machine_state"));

        let event = self
            .nostr_client
            .sign_event_builder(event_builder)
            .await
            .map_err(VendingMachineError::Nostr)?;
        self.nostr_client
            .send_event(&event)
            .await
            .map_err(VendingMachineError::Nostr)?;
        Ok(())
    }

    pub async fn update_last_activity(&mut self) -> Result<(), VendingMachineError> {
        self.last_activity = Some(Instant::now());
        self.send_update().await
    }

    pub async fn add_item(&mut self, item: Item) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if !self.under_admin {
            return Err(VendingMachineError::Unauthorized("only admin can add item"));
        }

        if let Some(state) = self.state.take() {
            self.state = Some(state.add_item(self, item)?);
            return Ok(());
        }

        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub async fn admin(&mut self) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if let Some(state) = self.state.take() {
            self.state = Some(state.admin(self)?);
        }
        Ok(())
    }

    pub async fn change_price(
        &mut self,
        item_id: u64,
        new_price: u64,
    ) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if let Some(state) = self.state.take() {
            self.state = Some(state.change_price(self, item_id, new_price)?);
        }
        Ok(())
    }

    pub async fn remove_item(&mut self, item_id: u64) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if let Some(state) = self.state.take() {
            self.state = Some(state.remove_item(self, item_id)?);
        }
        Ok(())
    }

    pub async fn request_item(&mut self, item_id: u64) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_item(self, item_id)?);
            return Ok(());
        }
        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub async fn insert_money(&mut self, money: u64) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if let Some(state) = self.state.take() {
            self.state = Some(state.insert_money(money)?);
            return Ok(());
        }
        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub async fn dispense_item(&mut self) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if let Some(state) = self.state.take() {
            self.state = Some(state.dispense_item(self)?);
            return Ok(());
        }
        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub async fn cancel(&mut self) -> Result<(), VendingMachineError> {
        self.update_last_activity().await?;
        if let Some(state) = self.state.take() {
            self.state = Some(state.cancel(self)?);
            return Ok(());
        }
        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub fn show_commands(&self) {
        self.state.as_ref().unwrap().show_commands()
    }

    pub fn show_items(&self) {
        println!("----------------------------------------------------------");
        println!("items: ");
        for item in self.items.iter() {
            println!(
                "id: {}, name: {}, price: {}, stock: {}",
                item.1.id, item.1.name, item.1.price, item.1.count
            )
        }
        println!("----------------------------------------------------------");
    }

    pub(crate) fn increment_item_count(
        &mut self,
        add_items: Item,
    ) -> Result<(), VendingMachineError> {
        if !self.under_admin {
            return Err(VendingMachineError::Unauthorized("only admin can add item"));
        }
        self.items
            .entry(add_items.id)
            .or_insert(Item::new(add_items.id, add_items.name, add_items.price, 0))
            .increment_count(add_items.count);
        Ok(())
    }

    pub(crate) fn remove_item_from_menu(
        &mut self,
        item_id: u64,
    ) -> Result<(), VendingMachineError> {
        if !self.under_admin {
            return Err(VendingMachineError::Unauthorized("only admin can add item"));
        }
        self.items
            .remove(&item_id)
            .map(|_| ())
            .ok_or(VendingMachineError::ItemDoesNotExist(item_id))
    }

    pub(crate) fn change_item_price(
        &mut self,
        item_id: u64,
        price: u64,
    ) -> Result<(), VendingMachineError> {
        if !self.under_admin {
            return Err(VendingMachineError::Unauthorized("only admin can add item"));
        }
        if let Some(item) = self.items.get_mut(&item_id) {
            item.price = price;
            return Ok(());
        }
        Err(VendingMachineError::ItemDoesNotExist(item_id))
    }

    pub fn get_item(&self, item_id: u64) -> Option<&Item> {
        self.items.get(&item_id)
    }

    pub(crate) fn sell_item_unit(&mut self, item_id: u64) {
        self.items
            .entry(item_id)
            .and_modify(|item| item.sell_unit());
    }

    // Process the next admin command if available
    pub async fn process_next_admin_command(
        &mut self,
        command: &AdminCommand,
    ) -> Result<bool, VendingMachineError> {
        // Process the admin command
        match command {
            AdminCommand::ChangePrice(change_price_req) => {
                self.change_price(change_price_req.id, change_price_req.price)
                    .await?;
                Ok(true)
            }
            AdminCommand::RemoveItem(item_id) => {
                self.remove_item(item_id.to_owned()).await?;
                Ok(true)
            }
            AdminCommand::RequestAdminState => {
                self.admin().await?;
                Ok(true)
            }
            AdminCommand::Reboot => {
                println!("Admin requested reboot");
                // Implement reboot logic
                Ok(true)
            }
            AdminCommand::Status => {
                println!("Admin requested status");
                self.show_items();
                Ok(true)
            }
            AdminCommand::AddItem(item_data) => {
                println!(
                    "Admin adding item: id={}, count={}",
                    item_data.id, item_data.count
                );
                self.add_item(Item {
                    id: item_data.id,
                    name: item_data.name.clone(),
                    price: item_data.price,
                    count: item_data.count,
                })
                .await?;
                Ok(true)
            }
            AdminCommand::Shutdown => {
                println!("Admin requested shutdown");
                Ok(true)
            }
            AdminCommand::End => {
                println!("Admin finished working");
                self.cancel().await?;
                Ok(true)
            }
        }
    }

    pub async fn run_machine(&mut self) -> Result<(), VendingMachineError> {
        loop {
            tokio::select! {
                Some(shutdown) = self.shutdown.recv() => {
                    if shutdown {
                        println!("Shutdown signal received. Exiting...");
                        break;
                    }
                }
                Some(command) = self.admin_commands.recv() => {
                    if let Err(e) = self.process_next_admin_command(&command).await {
                        eprintln!("Error processing admin command: {}", e);
                    }
                }
                _ = tokio::time::sleep(tokio::time::Duration::from_secs(5)) => {
                    if let Some(last_activity) = self.last_activity {
                        if last_activity.elapsed().as_secs() > 60 {
                            println!("No activity for 60 seconds. Cancelling...");
                            self.cancel().await?;
                            break;
                        }
                    }
                }
                else => {
                    if let Err(e) = self.process_user_input().await {
                        eprintln!("Error processing user input: {}", e);
                    }
                }
            }
        }

        Ok(())
    }

    async fn process_user_input(&mut self) -> Result<bool, VendingMachineError> {
        println!("==============================================================");
        self.show_commands();
        let code = helper::read_number("Select code (0 to exit): ");

        match code {
            1 => {
                self.show_items();
                let id = helper::read_number("write the id of the item (number): ");
                let mut name = String::new();
                let mut price = 0;
                if self.get_item(id).is_none() {
                    name = helper::read_string("write the name of the item (string): ");
                    price = helper::read_number("write the price of the item (number): ");
                }
                let count =
                    helper::read_number("write the quantity adding to the stock of that item: ");
                self.add_item(Item::new(id, name, price, count)).await?;
                self.show_items();
            }
            2 => {
                self.show_items();
                let id =
                    helper::read_number("requesting item. Provide the id of the item (number): ");
                self.request_item(id).await?;
            }
            3 => {
                let money = helper::read_number("insert money. Provide the amount (number): ");
                self.insert_money(money).await?;
            }
            4 => {
                self.dispense_item().await?;
                self.show_items();
            }
            5 => {
                self.cancel().await?;
            }
            _ => {
                println!("invalid code. Try again")
            }
        }

        Ok(true)
    }
}
