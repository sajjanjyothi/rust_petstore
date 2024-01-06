use actix_web::{get, post, web,  HttpResponse, Responder};
use crate::store;
use std::sync::Mutex;
use actix_web::web::Data;

#[get("/pets")]
pub async fn get_pets(pet_store:Data<Mutex<store::pets::PetStore>>) -> impl Responder {
    let store = pet_store.lock().unwrap();
    HttpResponse::Ok().json(store.get_pets())
}

#[post("/pets")]
pub async fn add_pet(pet:web::Json<store::pets::Pet>,pet_store:Data<Mutex<store::pets::PetStore>>) -> impl Responder {
   let mut store = pet_store.lock().unwrap();
    store.add_pet(pet.into_inner());
    HttpResponse::Ok().json(store.get_pets())
}