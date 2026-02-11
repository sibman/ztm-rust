// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

impl Adult {
    fn new(name: &str, age: u8) -> Result<Adult, String> {
        if age < 21 {
            return Err("The person not in leagal age (less then 21)".to_owned());
        }
        Ok(Adult {
            name: name.to_owned(),
            age,
        })
    }
}

fn print_details(adult: &Adult) {
    println!("Adult name : {} {} years of age", &adult.name, adult.age);
}
fn main() {
    match Adult::new("Andrey", 52) {
        Ok(adult) => print_details(&adult),
        Err(result) => println!("{}", result),
    };
    match Adult::new("Andy", 16) {
        Ok(adult) => print_details(&adult),
        Err(result) => println!("{}", result),
    };
}
