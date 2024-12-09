use chrono;
use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use std::str::FromStr;
use std::{env, vec};

use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    options::{ClientOptions, ResolverConfig},
    Client,
};

use crate::config;
use crate::models;
use crate::entity;

use rocket::fairing::{self, AdHoc};
use rocket::http::{ContentType, Status};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;

use rocket::futures::TryStreamExt;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("movie controller", |rocket| async {
        rocket.mount("/movies", routes![index, read])
    })
}

#[get("/")]
async fn index(
    db_client: &State<config::mongo::MongoClient>,
) -> (Status, Json<models::common::Response<Vec<entity::movie_entity::Movie>>>) {
    let mut response = models::common::Response{ 
        code: Status::Ok.code as u8, 
        status: Status::Ok.reason().unwrap().to_string(), 
        data: Some(vec![]), 
        errors: None,
    };

    let movies: mongodb::Collection<Document> = db_client
        .client
        .database("sample_mflix")
        .collection("movies");
    let movie_cursor = movies
        .find(
            doc! {},
            None,
        )
        .await;
    if let Err(ref e) = movie_cursor {
        response.errors = Some(format!("Error finding data: {e}"));
        return (Status::InternalServerError, Json(response));
    }
    let mut movie_cursor = movie_cursor.unwrap();

    let mut movies = vec![];
    while let Ok(Some(doc)) = movie_cursor.try_next().await {
        let deserialized_movie: Result<entity::movie_entity::Movie, bson::de::Error> = bson::from_bson(Bson::Document(doc));
        if let Err(ref e) = deserialized_movie {
            response.errors = Some(format!("Error deserializing movie: {e}"));
            return (Status::InternalServerError, Json(response));
        }
        movies.push(deserialized_movie.unwrap());
    }
    response.data = Some(movies);

    return (Status::Ok, Json(response));
}

#[get("/<id>")]
async fn read(
    db_client: &State<config::mongo::MongoClient>,
    id: &str,
) -> (Status, Json<models::common::Response<entity::movie_entity::Movie>>) {
    let mut response = models::common::Response{ 
        code: Status::Ok.code as u8, 
        status: Status::Ok.reason().unwrap().to_string(), 
        data: None, 
        errors: None,
    };
    let movies_coll: mongodb::Collection<Document> = db_client
        .client
        .database("sample_mflix")
        .collection("movies");
    let movie_obj_id = match ObjectId::from_str(&id) {
        Ok(id) => id,
        Err(e) => {
            response.errors = Some(format!("Error getting movie: {e}"));
            return (
                Status::BadRequest,
                Json(response),
            );
        }
    };

    match movies_coll
        .find_one(doc! { "_id": movie_obj_id }, None)
        .await
    {
        Ok(Some(movie)) => {
            let deserialized_movie: entity::movie_entity::Movie = match bson::from_bson(Bson::Document(movie)) {
                Ok(movie) => movie,
                Err(e) => {
                    response.errors = Some(format!("Error finding movie: {e}"));
                    return (
                        Status::InternalServerError,
                        Json(response)
                    );
                }
            };

            response.data = Some(deserialized_movie);
            return (Status::Ok, Json(response));
        }
        Ok(None) => {
            response.errors = Some(format!("No movie found with ID: {id}"));
            return (
                Status::NotFound,
                Json(response),
            );
        }
        Err(e) => {
            response.errors = Some(format!("Error finding movie: {e}"));
            return (
                Status::InternalServerError,
                Json(response),
            );
        }
    }
}