use crate::vm::listening_state::ListeningState;

use super::{state::State, vending_machine::VendingMachine};

pub struct AdminState;

impl AdminState {
    pub fn new() -> AdminState {
        Self {}
    }
}

impl State for AdminState {
    fn show_commands(&self) {}

    fn cancel(
        self: Box<Self>,
        vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, super::vending_machine::VendingMachineError> {
        println!("leaving admin state");
        vm.under_admin = false;
        Ok(Box::new(ListeningState))
    }

    fn add_item(
        self: Box<Self>,
        vm: &mut super::vending_machine::VendingMachine,
        item: super::vending_machine::Item,
    ) -> Result<Box<dyn State>, super::vending_machine::VendingMachineError> {
        vm.increment_item_count(item)?;
        Ok(self)
    }

    fn remove_item(
        self: Box<Self>,
        vm: &mut super::vending_machine::VendingMachine,
        item_id: u64,
    ) -> Result<Box<dyn State>, super::vending_machine::VendingMachineError> {
        vm.remove_item_from_menu(item_id)?;
        Ok(self)
    }

    fn change_price(
        self: Box<Self>,
        vm: &mut super::vending_machine::VendingMachine,
        item_id: u64,
        new_price: u64,
    ) -> Result<Box<dyn State>, super::vending_machine::VendingMachineError> {
        vm.change_item_price(item_id, new_price)?;
        Ok(self)
    }
}
