fn add(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

fn subtract(first_number: i32, second_number: i32) -> i32 {
    first_number - second_number
}

fn main() {
    let name = "Nikhil";
    println!("Hello {}", name);

    let first = 1;
    let second = 2;
    let addition = add(first, second);
    println!("Addition is {addition}");

    let subtraction = subtract(first, second);
    println!("Subtraction is {subtraction}");
}

#[cfg(test)]
mod tests {
    use crate::{add, subtract};
    #[test]
    fn add_test() {
        assert_eq!(3, add(1, 2));
    }
    #[test]
    fn add_subtract() {
        assert_eq!(-1, subtract(1, 2));
    }
}