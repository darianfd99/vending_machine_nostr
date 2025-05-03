use std::{collections::HashMap, fmt::Display, time::Duration};

use tokio::sync::mpsc;

use crate::{
    admin::{commands::AdminCommand, AdminError},
    helper,
    listening_state::ListeningState,
    state::State,
};

#[derive(Debug)]
pub enum VendingMachineError {
    OutOfStock(&'static str),
    AddItem(&'static str),
    Dispense(&'static str),
    InsertMoney(&'static str),
    RequestItem(&'static str),
    Unauthorized(&'static str),
    AdminError(AdminError),
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
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) price: u64,
    pub(crate) count: u64,
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

pub struct VendingMachine {
    under_admin: bool,
    state: Option<Box<dyn State>>,
    items: HashMap<u64, Item>,
    admin_commands: mpsc::Receiver<AdminCommand>,
}

impl VendingMachine {
    pub fn new(admin_commands: mpsc::Receiver<AdminCommand>) -> Self {
        Self {
            under_admin: false,
            state: Some(Box::new(ListeningState)),
            items: HashMap::new(),
            admin_commands,
        }
    }

    pub async fn listen_to_admins(&mut self) {
        while let Some(command) = self.admin_commands.recv().await {
            println!("Received: {:?}", command);
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), VendingMachineError> {
        if !self.under_admin {
            return Err(VendingMachineError::Unauthorized("only admin can add item"));
        }

        if let Some(state) = self.state.take() {
            self.state = Some(state.add_item(self, item)?);
            return Ok(());
        }

        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub fn request_item(&mut self, item_id: u64) -> Result<(), VendingMachineError> {
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_item(self, item_id)?);
            return Ok(());
        }
        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub fn insert_money(&mut self, money: u64) -> Result<(), VendingMachineError> {
        if let Some(state) = self.state.take() {
            self.state = Some(state.insert_money(money)?);
            return Ok(());
        }
        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub fn dispense_item(&mut self) -> Result<(), VendingMachineError> {
        if let Some(state) = self.state.take() {
            self.state = Some(state.dispense_item(self)?);
            return Ok(());
        }
        Err(VendingMachineError::AddItem("invalid state"))
    }

    pub fn cancel(&mut self) -> Result<(), VendingMachineError> {
        if let Some(state) = self.state.take() {
            self.state = Some(state.cancel()?);
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

    pub(crate) fn increment_item_count(&mut self, add_items: Item) {
        self.items
            .entry(add_items.id)
            .or_insert(Item::new(add_items.id, add_items.name, add_items.price, 0))
            .increment_count(add_items.count)
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
    pub async fn process_next_admin_command(&mut self) -> Result<bool, VendingMachineError> {
        // Try to receive a command with a very short timeout
        match tokio::time::timeout(Duration::from_millis(10), self.admin_commands.recv()).await {
            Ok(Some(command)) => {
                // Process the admin command
                match command {
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

                        // Check if item exists before attempting to add it
                        let item_option = self.get_item(item_data.id).map(|existing| {
                            Item::new(
                                item_data.id,
                                existing.name.clone(),
                                existing.price,
                                item_data.count,
                            )
                        });

                        if let Some(item) = item_option {
                            // Use the with_admin_privileges function
                            self.with_admin_privileges(|vm| vm.add_item(item))
                                .map(|_| true)
                        } else {
                            // Item doesn't exist yet
                            Err(VendingMachineError::AddItem(
                                "Cannot add new item; only update existing",
                            ))
                        }
                    }
                }
            }
            Ok(None) => {
                // Channel closed
                println!("Admin command channel closed");
                Ok(false)
            }
            Err(_) => {
                // Timeout, no command available
                Ok(false)
            }
        }
    }

    pub async fn run_machine(&mut self) -> Result<(), VendingMachineError> {
        loop {
            // Check for admin commands (non-blocking)
            match self.process_next_admin_command().await {
                Ok(true) => continue, // Command was processed, continue loop
                Ok(false) => {}       // No admin command to process
                Err(e) => eprintln!("Error processing admin command: {}", e),
            }

            // Process user input (potentially blocking)
            if self.process_user_input().await? {
                break; // User requested exit
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
                self.add_item(Item::new(id, name, price, count))?;
                self.show_items();
            }
            2 => {
                self.show_items();
                let id =
                    helper::read_number("requesting item. Provide the id of the item (number): ");
                self.request_item(id)?;
            }
            3 => {
                let money = helper::read_number("insert money. Provide the amount (number): ");
                self.insert_money(money)?;
            }
            4 => {
                self.dispense_item()?;
                self.show_items();
            }
            5 => {
                self.cancel()?;
            }
            _ => {
                println!("invalid code. Try again")
            }
        }

        Ok(true)
    }

    fn with_admin_privileges<F, T>(&mut self, operation: F) -> T
    where
        F: FnOnce(&mut Self) -> T,
    {
        let previous = self.under_admin;
        self.under_admin = true;
        let result = operation(self);
        self.under_admin = previous;
        result
    }
}
