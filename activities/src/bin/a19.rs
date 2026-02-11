// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

struct Store {
    items: HashMap<String, u8>,
}

impl Store {
    fn new() -> Self {
        Self{
            items: HashMap::new(),
        }
    }
    fn add_item(&mut self, name: &str, items_count: u8) {
        self.items.insert(name.to_owned(), items_count);
    }
    fn get_count(&self, name: &str) -> Option<&u8> {
        self.items.get(name)
    }
    fn print_details(&self, name: &str) {
        match self.get_count(name) {
            Some(0) => println!("out of stock"),
            Some(num) => println!("Number of items {}", num),
            None => println!("Item does not exist"),
        }
    }
}
fn main() {
    let mut store = Store::new();
    store.add_item("Chairs", 5);
    store.add_item("Beds", 3);
    store.add_item("Tables", 2);
    store.add_item("Couches", 0);

    store.print_details("Chairs");
    store.print_details("Couches");
    store.print_details("Futons");
}
