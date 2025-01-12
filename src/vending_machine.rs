use std::{collections::HashMap, fmt::Display};

use crate::{listening_state::ListeningState, state::State};

#[derive(Debug)]
pub enum VendingMachineError {
    OutOfStock(&'static str),
    AddItem(&'static str),
    Dispense(&'static str),
    InsertMoney(&'static str),
    RequestItem(&'static str),
}

impl Display for VendingMachineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::OutOfStock(s) => write!(f, "VendingMachineError::OutOfStock: {}", s),
            Self::Dispense(s) => write!(f, "VendingMachineError::DispenseWithoutSelection: {}", s),
            Self::InsertMoney(s) => write!(f, "VendingMachineError::InsertMoney: {}", s),
            Self::AddItem(s) => write!(f, "VendingMachineError::AddItem: {}", s),
            Self::RequestItem(s) => write!(f, "VendingMachineError::RequestItem: {}", s),
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
    state: Option<Box<dyn State>>,
    items: HashMap<u64, Item>,
}

impl Default for VendingMachine {
    fn default() -> Self {
        Self::new()
    }
}

impl VendingMachine {
    pub fn new() -> Self {
        Self {
            state: Some(Box::new(ListeningState)),
            items: HashMap::new(),
        }
    }

    pub fn add_item(&mut self, item: Item) -> Result<(), VendingMachineError> {
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
}
