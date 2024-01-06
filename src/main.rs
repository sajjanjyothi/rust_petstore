mod store;
mod routes;
use std::sync::Mutex;
use actix_web::web::Data;
use actix_web::middleware::Logger;
use env_logger::Env;
use actix_web::{ App, HttpServer};
use routes::pets::{get_pets,add_pet};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let data = Data::new(Mutex::new(store::pets::PetStore::new()));
    HttpServer::new( move|| {
        App::new()
            .wrap(Logger::default())
            .app_data(Data::clone(&data))
            .service(get_pets)
            .service(add_pet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
