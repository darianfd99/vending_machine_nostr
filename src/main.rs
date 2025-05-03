use std::io;

use nostr_sdk::Keys;
use vending_machines_nostr::{
    admin::{builder::AdminHandlerBuilder, commands::AdminCommand},
    Item, VendingMachine, VendingMachineError,
};

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

#[tokio::main]
async fn main() -> Result<(), VendingMachineError> {
    // Generate new random keys
    let keys = Keys::generate();
    let nostr_client = nostr_sdk::ClientBuilder::new().signer(keys.clone()).build();

    nostr_client
        .add_relay("wss://relay.damus.io")
        .await
        .unwrap();

    nostr_client
        .add_read_relay("wss://relay.nostr.info")
        .await
        .unwrap();

    nostr_client.connect().await;

    let (admin_tx, admin_rx) = tokio::sync::mpsc::channel::<AdminCommand>(100);
    let admin_handler = AdminHandlerBuilder::new()
        .private_key(keys.secret_key().clone())
        .add_admin_pubkey("npub1fvzv9hfgxk45n2fzr5dnqu5jxza58vj5ulh9sqq2xf8ryg2h2a6sxgpg4z")
        .unwrap()
        .client(nostr_client)
        .sender_admin_commands(admin_tx)
        .build()
        .unwrap();

    admin_handler.subscribe().await;

    tokio::spawn(async move {
        let _ = admin_handler.handle_events().await;
    });

    let mut vm = VendingMachine::new(admin_rx);
    loop {
        println!("==============================================================");
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
                vm.show_items();
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
