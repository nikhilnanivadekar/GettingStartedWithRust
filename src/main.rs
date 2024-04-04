mod domain;

use std::collections::HashMap;


fn add(first_number: i32, second_number: i32) -> i32 {
    first_number + second_number
}

fn subtract(first_number: i32, second_number: i32) -> i32 {
    first_number - second_number
}

fn get_full_name(first_name: &String, last_name: &String) -> String {
    format!("{first_name} {last_name}")
}

fn main() {
    let first_name = "Nikhil";
    println!("Hello {}", first_name);

    let first = 1;
    let second = 2;
    let addition = add(first, second);
    println!("Addition is {addition}");

    let subtraction = subtract(first, second);
    println!("Subtraction is {subtraction}");

    let first_name = String::from("Nikhil");
    let last_name = String::from("Nanivadekar");
    let full_name = get_full_name(&first_name, &last_name);

    println!("First Name is {first_name}");
    println!("Last Name is {last_name}");
    println!("Full Name is {full_name}");

    let mut conferences = HashMap::new();
    conferences.insert("Jfokus", "Sweden");
    conferences.insert("Javaland", "Germany");
    conferences.insert("Devnexus", "USA");

    println!("Conferences which Nikhil has presented so far in 2024 are {:?}", conferences);
    let javaland_country = conferences.get("Javaland").unwrap();
    println!("Javaland is in {javaland_country}");
}

#[cfg(test)]
mod tests {
    use crate::{add, get_full_name, subtract};

    #[test]
    fn add_test() {
        assert_eq!(3, add(1, 2));
    }

    #[test]
    fn add_subtract() {
        assert_eq!(-1, subtract(1, 2));
    }

    #[test]
    fn get_full_name_test() {
        assert_eq!("Nikhil Nanivadekar", get_full_name(&String::from("Nikhil"), &String::from("Nanivadekar")));
    }
}