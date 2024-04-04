#[derive(PartialEq, Debug, Clone)]
pub(crate) struct Pet {
    pub(crate) name: String,
    pub(crate) age: i32,
    pub(crate) pet_type: PetType,
}

#[derive(PartialEq, Debug, Clone)]
pub(crate) enum PetType {
    DOG,
    CAT,
}
