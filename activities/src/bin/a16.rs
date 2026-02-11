// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

#[derive(Debug)]
struct Student {
    name: String,
    locker: Option<i32>,
}

impl Student {
    fn new(name: String, locker: Option<i32>) -> Self {
        Self { name, locker }
    }
    fn print_details(&self) {
        println!("Name: {}", self.name);
        match &self.locker {
            Some(locker) => println!("Locker: {}", locker),
            None => println!("No locker assigned"),
        }
    }
}

fn main() {
    let student1 = Student::new("John".to_string(), Some(123));
    student1.print_details();
    let student2 = Student::new("Jane".to_string(), None);
    student2.print_details();
    let student3 = Student::new("Bob".to_string(), Some(456));
    student3.print_details();
}
