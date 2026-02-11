// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

#[derive(Debug)]
enum Ticket {
    Backstage { name: String, price: f64 },
    Vip { name: String, price: f64 },
    Standard { price: f64 },
}

fn print_ticket(ticket: &Ticket) {
    match ticket {
        Ticket::Backstage { name, price } => {
            println!("Backstage ticket for {} at ${}", name, price);
        }
        Ticket::Vip { name, price } => {
            println!("Vip ticket for {} at ${}", name, price);
        }
        Ticket::Standard { price } => {
            println!("Standard ticket at ${}", price);
        }
    }
}

fn main() {
    let tickets = vec![
        Ticket::Backstage {
            name: "John".to_string(),
            price: 100.0,
        },
        Ticket::Vip {
            name: "Jane".to_string(),
            price: 200.0,
        },
        Ticket::Standard { price: 300.0 },
    ];
    for ticket in tickets {
        print_ticket(&ticket);
    }
}
