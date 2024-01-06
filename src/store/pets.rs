use serde::{Serialize,Deserialize};

pub struct PetStore {
    pets: Option<Vec<Pet>>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
pub struct Pet{
    name: String,
    pet_type: String,
    age: i8,
}

#[allow(dead_code)]
impl Pet {
    pub fn new(name: String, pet_type: String, age: i8) -> Self {
        Self {
            name,
            pet_type,
            age,
        }
    }
}

impl PetStore {
    pub fn new() -> Self {
        Self {
            pets: Some(Vec::new()),
        }
    }

    pub fn add_pet(&mut self, pet: Pet) {
       if let Some(pets) = &mut self.pets {
           pets.push(pet);
       } 
    }

    pub fn get_pets(&self) -> &Vec<Pet> {
       self.pets.as_ref().unwrap()
    }
}

    
