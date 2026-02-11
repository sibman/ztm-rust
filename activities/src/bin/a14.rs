// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    favorite_color: String,
}

fn print_person(person: &Person) {
    println!("Name: {}", person.name);
    println!("Age: {}", person.age);
    println!("Favorite color: {}", person.favorite_color);
}

fn main() {
    let person1 = Person {
        name: "John".to_string(),
        age: 10,
        favorite_color: "Red".to_string(),
    };
    let person2 = Person {
        name: "Jane".to_string(),
        age: 15,
        favorite_color: "Blue".to_string(),
    };
    let person3 = Person {
        name: "Bob".to_string(),
        age: 20,
        favorite_color: "Green".to_string(),
    };
    let people = vec![person1, person2, person3];
    for person in people {
        if person.age <= 10 {
            print_person(&person);
        }
        println!();
    }
}
