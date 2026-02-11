// Topic: Browsing standard library documentation
//
// Requirements:
// * Print a string in lowercase and uppercase
//
// Notes:
// * Utilize standard library functionality to
//   transform the string to lowercase and uppercase
// * Use 'rustup doc' in a terminal to open the standard library docs
// * Navigate to the API documentation section
// * Search for functionality to transform a string (or str)
//   to uppercase and lowercase
//   * Try searching for: to_uppercase, to_lowercase

use std::io;

fn main() {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err() {
        print!("Please enter your data again");
    }
    let upper = buffer.to_uppercase();
    let lower = buffer.to_lowercase();

    println!("Upper case: {}, lower case: {}", upper, lower);
}
