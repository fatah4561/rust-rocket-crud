#[macro_use]
extern crate rocket;

use dotenv::dotenv;

mod config;
mod controller;
mod models;
mod entity;
mod service;
mod repository;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client = config::mongo::new().await;
    if let Err(ref e) = client {
        eprintln!("Error connecting to db: {e}")
    }
    let client = client.unwrap();

    rocket::build()
        .attach(controller::movie_controller::stage(client))
}
