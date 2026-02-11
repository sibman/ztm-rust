// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{clamp, div, concat};

    #[test]
    fn clamp_test() {
        assert_eq!(clamp(5, 3, 7), 5);
        assert_eq!(clamp(2, 3, 7), 3);
        assert_eq!(clamp(10, 3, 7), 7);
    }
    #[test]
    fn div_test() {
        assert_eq!(div(10, 2), Some(5));
        assert_eq!(div(10, 3), Some(3));
        assert_eq!(div(10, 4), Some(2));
        assert_eq!(div(10, 0), None);

    }
    #[test]
    fn concat_test() {
        assert_eq!(concat("hello", "world"), "hello world");
        assert_eq!(concat("hello", "rust"), "hello rust");
        assert_eq!(concat("rust", "is"), "rust is");
    }
}
