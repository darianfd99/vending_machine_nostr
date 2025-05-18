use crate::vm::listening_state::ListeningState;

use super::vending_machine::{Item, VendingMachine, VendingMachineError};

pub(crate) trait State: Send + Sync {
    // user commands
    fn request_item(
        self: Box<Self>,
        _vm: &VendingMachine,
        _item_id: u64,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Unauthorized(
            "Cannot request item in current state",
        ))
    }

    fn insert_money(self: Box<Self>, _money: u64) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Unauthorized(
            "Cannot insert moneey in current state",
        ))
    }
    fn dispense_item(
        self: Box<Self>,
        _vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Unauthorized(
            "Cannot dispense item in current state",
        ))
    }

    // generics
    fn show_commands(&self);
    fn cancel(
        self: Box<Self>,
        _vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        println!("cancel");
        Ok(Box::new(ListeningState))
    }

    // admin methods
    fn admin(
        self: Box<Self>,
        _vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Unauthorized(
            "Cannot go to admin in current state",
        ))
    }
    fn add_item(
        self: Box<Self>,
        _vm: &mut VendingMachine,
        _item: Item,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Unauthorized(
            "Cannot add items in current state",
        ))
    }
    fn remove_item(
        self: Box<Self>,
        _vm: &mut VendingMachine,
        _item_id: u64,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Unauthorized(
            "Cannot remove items in current state",
        ))
    }
    fn change_price(
        self: Box<Self>,
        _vm: &mut VendingMachine,
        _item_id: u64,
        _new_price: u64,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Unauthorized(
            "Cannot change price in current state",
        ))
    }
}
