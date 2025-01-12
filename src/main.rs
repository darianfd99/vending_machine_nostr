use std::io;

use vending_machines_nostr::{Item, VendingMachine, VendingMachineError};

fn read_number(text: &str) -> u64 {
    println!("{}", text);
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read the line");
    num.trim().parse().expect("failed to read number")
}

fn read_string(text: &str) -> String {
    println!("{}", text);
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read the line");
    name.trim().to_string()
}

fn main() -> Result<(), VendingMachineError> {
    let mut vm = VendingMachine::new();
    loop {
        vm.show_commands();
        let code = read_number("Select code: ");
        match code {
            1 => {
                vm.show_items();
                let id = read_number("write the id of the item (number): ");
                let mut name = String::new();
                let mut price = 0;
                if vm.get_item(id).is_none() {
                    name = read_string("write the name of the item (string): ");
                    price = read_number("write the price of the item (number): ");
                }
                let count = read_number("write the quantity adding to the stock of that item: ");
                vm.add_item(Item::new(id, name, price, count))?;
            }
            2 => {
                vm.show_items();
                let id = read_number("requesting item. Provide the id of the item (number): ");
                vm.request_item(id)?;
            }
            3 => {
                let money = read_number("insert money. Provide the amount (number): ");
                vm.insert_money(money)?;
            }
            4 => {
                vm.dispense_item()?;
                vm.show_items();
            }
            5 => {
                vm.cancel()?;
            }
            _ => {
                println!("invalid code. Try again")
            }
        }
    }
}
