use serde::{Serialize,Deserialize, de};
use log::{debug};

#[derive(Debug)] 
#[derive(PartialEq)]
pub struct PetStore {
    pets: Option<Vec<Pet>>,
}

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(PartialEq)]
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
        debug!("Adding pet {:?}", pet);
        pets.push(pet);  
       } 
    }

    pub fn get_pets(&self) -> &Vec<Pet> {
       self.pets.as_ref().unwrap()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_pet() {
        let mut store = PetStore::new();
        let pet = Pet::new("Rusty".to_string(), "Dog".to_string(), 8);
        store.add_pet(pet);
        assert_eq!(store.get_pets().len(), 1);
        assert_eq!(store.get_pets(), &vec![Pet::new("Rusty".to_string(), "Dog".to_string(), 8)]);
    }
}

