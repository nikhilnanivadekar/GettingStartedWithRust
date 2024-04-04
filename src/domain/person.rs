use crate::domain::pet::{Pet, PetType};

#[derive(PartialEq, Debug)]
pub struct Person {
    pub first_name: String,
    pub last_name: String,
    pub pets: Vec<Pet>,
}

pub trait PersonOperations {
    fn has_pet(&self, pet_type: PetType) -> bool;
}

impl PersonOperations for Person {
    fn has_pet(&self, pet_type: PetType) -> bool {
        self.pets.iter().any(|pet| pet.pet_type == pet_type)
    }
}
