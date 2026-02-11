// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

#[derive(Debug)]
enum DrinkFlavor {
    Bitter,
    Savory,
    Sour,
    Sweet,
}

#[derive(Debug)]
struct DrinkInfo {
    flavor: DrinkFlavor,
    fluid_ounce: u32,
}

fn print_drink(drink: &DrinkInfo) {
    println!("Drink Information: {:?}", drink);
    match &drink.flavor {
        DrinkFlavor::Bitter => println!("Bitter Drink"),
        DrinkFlavor::Savory => println!("Savory Drink"),
        DrinkFlavor::Sour => println!("Sour Drink"),
        DrinkFlavor::Sweet => println!("Sweet Drink"),
    }
    println!("Drink flavor: {:?}", &drink.flavor);
    println!("Fluid ounces: {:?}", &drink.fluid_ounce);
}

fn main() {
    let drink1 = DrinkInfo {
        flavor: DrinkFlavor::Bitter,
        fluid_ounce: 15,
    };
    print_drink(&drink1);
    let drink2 = DrinkInfo {
        flavor: DrinkFlavor::Sweet,
        fluid_ounce: 20,
    };
    print_drink(&drink2);
    let drink3 = DrinkInfo {
        flavor: DrinkFlavor::Savory,
        fluid_ounce: 20,
    };
    print_drink(&drink3);
    let drink4 = DrinkInfo {
        flavor: DrinkFlavor::Sour,
        fluid_ounce: 20,
    };
    print_drink(&drink4);
}
