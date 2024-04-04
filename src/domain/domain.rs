use crate::domain::person::Person;
use crate::domain::pet::Pet;

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
    use crate::domain::domain::{add_pet};
    use crate::domain::person::{Person, PersonOperations};
    use crate::domain::pet::{Pet, PetType};

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
