use crate::{
    has_money_state::HasMoneyState,
    listening_state::ListeningState,
    state::State,
    vending_machine::{Item, VendingMachine, VendingMachineError},
};

pub(crate) struct ItemRequestedState {
    item: Item,
}

impl ItemRequestedState {
    pub fn new(item: Item) -> Self {
        Self { item }
    }
}

impl State for ItemRequestedState {
    fn add_item(
        self: Box<Self>,
        _vm: &mut VendingMachine,
        _item: Item,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::AddItem("Item dispense in progress"))
    }

    fn request_item(
        self: Box<Self>,
        _vm: &VendingMachine,
        _item_id: u64,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::RequestItem("Requested another item"))
    }

    fn insert_money(self: Box<Self>, money: u64) -> Result<Box<dyn State>, VendingMachineError> {
        if money != self.item.price {
            println!(
                "Inserted wrong amount: {} units, please insert {} units",
                money, self.item.price,
            );
            return Ok(self);
        }
        println!("Money intered is ok: {} units", money);
        Ok(Box::new(HasMoneyState::new(self.item.id, money)))
    }

    fn dispense_item(
        self: Box<Self>,
        _vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::Dispense("Insert money first"))
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, VendingMachineError> {
        println!("cancel");
        Ok(Box::new(ListeningState))
    }

    fn show_commands(&self) {
        println!("Commands: (3) insertMoney (5) cancel");
    }
}
