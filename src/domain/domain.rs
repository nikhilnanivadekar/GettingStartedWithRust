#[derive(PartialEq, Debug, Clone)]
pub(crate) enum PetType {
    DOG,
    CAT,
}

#[derive(PartialEq, Debug)]
struct Person {
    first_name: String,
    last_name: String,
    pets: Vec<Pet>,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Pet {
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) pet_type: PetType,
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
    use crate::domain::domain::{add_pet, Person, PersonOperations, Pet, PetType};

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
