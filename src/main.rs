#[macro_use]
extern crate rocket;

use std::sync::Arc;

use dotenv::dotenv;

mod config;
mod controller;
mod entity;
mod exception;
mod helper;
mod models;
mod repository;
mod service;

use rust_crud_mongo::not_found;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client = config::mongo::new().await;
    if let Err(ref e) = client {
        eprintln!("Error connecting to db: {e}")
    }
    let client = client.unwrap();

    // repository here
    let movie_repository: Arc<repository::movie_repository::MovieRepository> =
        repository::movie_repository::new_movie_repository(client.client).into();

    // service here
    let movie_service: Arc<service::movie_service::MovieService> =
        service::movie_service::new_movie_service(movie_repository).into();

    // controller here (controller used once so don't use Arc<>)
    let movie_controller = controller::movie_controller::new_movie_controller(movie_service);

    rocket::build()
    .register("/", catchers![not_found])
    .attach(controller::movie_controller::stage(movie_controller))
}