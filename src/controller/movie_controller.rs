use crate::config;
use crate::models;
use crate::entity;
use crate::repository;
use crate::repository::movie_repository::MovieRepository;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

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

    let movie_repo = repository::movie_repository::new_movie_repository(&db_client.client);
    let movies = movie_repo.get_all().await;

    if let Err(ref e) = movies {
        response.code = Status::InternalServerError.code as u8;
        response.status = Status::InternalServerError.reason().unwrap().to_string();
        response.errors = Some(e.to_string());
        return (Status::InternalServerError, Json(response))
    }
    let movies = movies.unwrap();
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

    let movie_repo = repository::movie_repository::new_movie_repository(&db_client.client);
    let movie = movie_repo.get_detail(id.to_string()).await;

    match movie {
        Ok(movie) => {
            response.data = Some(movie);
            return (Status::Ok, Json(response))
        },
        Err(e) => {
            response.errors = Some(format!("{e}"));
            return (Status::BadRequest, Json(response))
        }
    }
}