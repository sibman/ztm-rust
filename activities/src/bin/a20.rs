// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

enum PowerOptions {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

impl PowerOptions {
    fn from_str(input: &str) -> Option<Self> {
        match input {
            "off" => Some(Self::Off),
            "sleep" => Some(Self::Sleep),
            "reboot" => Some(Self::Reboot),
            "shutdown" => Some(Self::Shutdown),
            "hibernate" => Some(Self::Hibernate),
            _ => None,
        }
    }
    fn print_message(&self) {
        match self {
            Self::Off => println!("shutting down"),
            Self::Sleep => println!("sleeping"),
            Self::Reboot => println!("rebooting"),
            Self::Shutdown => println!("shutting down"),
            Self::Hibernate => println!("hibernating"),
        }
    }
}

fn run_program() {
    use std::io;

    print_power_options();
    let mut buffer = String::new();
    loop {
        println!("");
        print!("Enter selection: ");
        println!("");
        buffer.clear();
        while io::stdin().read_line(&mut buffer).is_err() {
            print!("Please enter your data again");
        }
        let input = buffer.trim().to_lowercase();
        let power_option = PowerOptions::from_str(input.as_str());
        match power_option {
            Some(power_option) => power_option.print_message(),
            None => println!("Invalid input"),
        }
    }
}

fn print_power_options() {
    println!("Enter power option:");
    println!("Off");
    println!("Sleep");
    println!("Reboot");
    println!("Shutdown");
    println!("Hibernate");
}

fn main() {
    run_program();
}
