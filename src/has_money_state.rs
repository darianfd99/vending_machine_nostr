use crate::{
    listening_state::ListeningState, state::State, vending_machine::VendingMachineError, Item,
    VendingMachine,
};

pub(crate) struct HasMoneyState {
    paid_item_id: u64,
    money: u64,
}

impl HasMoneyState {
    pub fn new(paid_item_id: u64, money: u64) -> Self {
        Self {
            paid_item_id,
            money,
        }
    }
}

impl State for HasMoneyState {
    fn request_item(
        self: Box<Self>,
        _vm: &VendingMachine,
        _item_id: u64,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::RequestItem(
            "Item dispense in progress",
        ))
    }

    fn add_item(
        self: Box<Self>,
        _vm: &mut VendingMachine,
        _item: Item,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::AddItem("Item dispense in progress"))
    }

    fn insert_money(self: Box<Self>, _money: u64) -> Result<Box<dyn State>, VendingMachineError> {
        Err(VendingMachineError::InsertMoney(
            "Item dispense in progress",
        ))
    }

    fn dispense_item(
        self: Box<Self>,
        vm: &mut VendingMachine,
    ) -> Result<Box<dyn State>, VendingMachineError> {
        println!(
            "Dispensing Item {} (id: {})",
            vm.get_item(self.paid_item_id).unwrap().name,
            self.paid_item_id
        );

        vm.sell_item_unit(self.paid_item_id);
        Ok(Box::new(ListeningState))
    }

    fn cancel(self: Box<Self>) -> Result<Box<dyn State>, VendingMachineError> {
        println!("paying back money: {} units", self.money);
        println!("cancel");
        Ok(Box::new(ListeningState))
    }

    fn show_commands(&self) {
        println!("Commands: (4) dispenseItem (5) cancel");
    }
}
