// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    let num_1 = 2;
    let num_2 = 5;
    let result = add_two_numbers(num_1, num_2);
    display_result(result);
}

fn add_two_numbers(arg_1: i32, arg_2: i32) -> i32 {
    arg_1 + arg_2
}

fn display_result(result: i32) {
    println!("result is: {:?}", result);
}
