// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
struct GroceryItem {
    quantity: i32,
    id: i32,
    name: String,
}

fn print_quantity(item: &GroceryItem) {
    println!("Quantity: {}", item.quantity);
}

fn print_id(item: &GroceryItem) {
    println!("ID: {}", item.id);
}

fn print_name(item: &GroceryItem) {
    println!("Name: {}", item.name);
}

fn main() {
    let item = GroceryItem {
        quantity: 5,
        id: 101,
        name: "Apples".to_string(),
    };
    print_quantity(&item);
    print_id(&item);
    print_name(&item);
    let item = GroceryItem {
        quantity: 15,
        id: 102,
        name: "Peanuts".to_string(),
    };
    print_quantity(&item);
    print_id(&item);
    print_name(&item);
    let item = GroceryItem {
        quantity: 25,
        id: 103,
        name: "Potatoes".to_string(),
    };
    print_quantity(&item);
    print_id(&item);
    print_name(&item);
}
