use std::collections::HashMap;

mod domain {
    #[derive(PartialEq, Debug, Clone)]
    enum PetType {
        DOG,
        CAT
    }

    #[derive(PartialEq, Debug)]
    struct Person {
        first_name: String,
        last_name: String,
        pets: Vec<Pet>,
    }

    #[derive(PartialEq, Debug, Clone)]
    struct Pet {
        name: String,
        age: i32,
        pet_type: PetType,
    }

    trait PersonOperations {
        fn has_pet(&self, pet_type: PetType) -> bool;
    }

    impl PersonOperations for Person {
        fn has_pet(&self, pet_type: PetType) -> bool {
            self.pets.iter().any(|pet| pet.pet_type == pet_type)
        }
    }

    fn add_pet(person: Person, pet: Pet) -> Person {
        let mut pets: Vec<Pet> = person.pets;
        pets.push(pet);
        let pets = pets;
        Person {
            pets,
            ..person
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::domain::{add_pet, Person, PersonOperations, Pet, PetType};


        #[test]
        fn has_pet_test() {
            let mut expected_pets = Vec::new();
            expected_pets.push(Pet { name: String::from("Dolly"), age: 2, pet_type: PetType::DOG });

            let person = Person { pets: expected_pets, first_name: String::from("Nikhil"), last_name: String::from("Nanivadekar") };

            assert!(person.has_pet(PetType::DOG));
            assert!(!person.has_pet(PetType::CAT));
        }
        #[test]
        fn add_pet_test() {
            let person = Person { first_name: String::from("Nikhil"), last_name: String::from("Nanivadekar"), pets: Vec::new() };

            let person1 = add_pet(person, Pet { name: String::from("Dolly"), age: 2, pet_type: PetType::DOG });

            let mut expected_pets = Vec::new();
            expected_pets.push(Pet { name: String::from("Dolly"), age: 2, pet_type: PetType::DOG });

            assert_eq!(Person { pets: expected_pets.clone(), first_name: String::from("Nikhil"), last_name: String::from("Nanivadekar") }, person1);

            let person1 = add_pet(person1, Pet { name: String::from("Holly"), age: 3, pet_type: PetType::CAT });
            expected_pets.push(Pet { name: String::from("Holly"), age: 3, pet_type: PetType::CAT });

            assert_eq!(Person { pets: expected_pets, first_name: String::from("Nikhil"), last_name: String::from("Nanivadekar") }, person1);


        }
    }
}

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