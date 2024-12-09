#[macro_use]
extern crate rocket;

use chrono;
use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use rocket::State;
use std::str::FromStr;
use std::{env, vec};

use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    options::{ClientOptions, ResolverConfig},
    Client,
};

use rocket::http::{ContentType, Status};
use rocket::serde::{json::Json, Deserialize, Serialize};

mod config;
mod controller;
mod models;
mod entity;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    let client = config::mongo::new().await;
    if let Err(ref e) = client {
        eprintln!("Error connecting to db: {e}")
    }
    let client = client.unwrap();

    // println!("Databases:");
    // let databases_name = client.list_database_names(None, None).await;
    // if let Err(ref e) = databases_name {
    //     eprintln!("Failed to get databases name: {e}")
    // }

    // for name in databases_name.unwrap() {
    //     println!("- {}", name);
    // }

    // let new_doc = doc!{
    //     "title": "Parasite",
    //     "year": 2024,
    //     "plot": "lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum lorem ipsum",
    //     "released": bson::DateTime::from_chrono( Utc.with_ymd_and_hms(2024, 2, 7, 0, 0, 0).single().unwrap()),
    // };

    // println!("{new_doc}");

    // let movies = client.database("sample_mflix").collection("movies");

    // let insert_movie = movies.insert_one(new_doc, None).await;
    // if let Err(ref e) = insert_movie {
    //     eprintln!("Failed to insert movie: {e}");
    // }
    // let insert_movie = insert_movie.unwrap();

    // println!("New document id: {}", insert_movie.inserted_id);

    // let movie = movies.find_one(doc!{
    //     "title": "Parasite",
    // }, None).await;
    // if let Err(ref e) = movie {
    //     eprintln!("Error finding one: {e}");
    // }
    // let movie = movie.unwrap().unwrap();

    // println!("Movie: {movie}");

    rocket::build()
        .manage(client)
        .attach(controller::movie_controller::stage())
}
