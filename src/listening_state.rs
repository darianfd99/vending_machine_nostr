use crate::{
    item_requested_state::ItemRequestedState,
    state::State,
    vending_machine::{Item, VendingMachine, VendingMachineError},
};

pub(crate) struct ListeningState;

impl State for ListeningState {
    fn add_item(
        self: Box<Self>,
        vm: &mut VendingMachine,
        item: Item,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        vm.increment_item_count(item);
        Ok(self)
    }

    fn request_item(
        self: Box<Self>,
        vm: &VendingMachine,
        item_id: u64,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        if let Some(item) = vm.get_item(item_id) {
            if item.count == 0 {
                println!("Item {} (id: {}) out of stock", item.name, item.id);
                return Ok(self);
            }
            println!("Item id:{} - name:{} requested", item_id, item.name);
            return Ok(Box::new(ItemRequestedState::new(item.clone())));
        }
        println!("invalid item id: {}", item_id);
        Ok(self)
    }

    fn dispense_item(
        self: Box<Self>,
        _vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Dispense("Request item first"))
    }

    fn insert_money(self: Box<Self>, _money: u64) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::InsertMoney("Request item first"))
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, VendingMachineError> {
        Ok(self)
    }

    fn show_commands(&self) {
        println!("Commands: (1) addItem (2) requestItem");
    }
}
