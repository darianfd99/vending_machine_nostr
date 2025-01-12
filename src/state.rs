use crate::vending_machine::{Item, VendingMachine, VendingMachineError};

pub(crate) trait State {
    fn add_item(
        self: Box<Self>,
        vm: &mut VendingMachine,
        item: Item,
    ) -> Result<Box<dyn State>, VendingMachineError>;
    fn request_item(
        self: Box<Self>,
        vm: &VendingMachine,
        item_id: u64,
    ) -> Result<Box<dyn State>, VendingMachineError>;
    fn insert_money(self: Box<Self>, money: u64) -> Result<Box<dyn State>, VendingMachineError>;
    fn dispense_item(
        self: Box<Self>,
        vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, VendingMachineError>;

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, VendingMachineError>;
    fn show_commands(&self);
}
