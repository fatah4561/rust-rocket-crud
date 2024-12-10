use crate::config;
use crate::models;
use crate::repository;
use crate::service;
use crate::service::movie_service::MovieServiceTrait;

use rocket::fairing::AdHoc;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

pub fn stage(client: config::mongo::MongoClient) -> AdHoc {
    AdHoc::on_ignite("movie controller", |rocket| async {
        rocket.manage(client).mount("/movies", routes![index, read])
    })
}

#[get("/")]
async fn index(
    db_client: &State<config::mongo::MongoClient>,
) -> (Status, Json<models::common_model::Response<Vec<models::movie_model::GetAllMoviesResponse>>>) {
    let mut response = models::common_model::Response{ 
        code: Status::Ok.code as u8, 
        status: Status::Ok.reason().unwrap().to_string(), 
        data: Some(vec![]), 
        errors: None,
    };

    let movie_repo = repository::movie_repository::new_movie_repository(&db_client.client);
    let movie_service = service::movie_service::new_movie_service(&movie_repo);

    let movies = movie_service.get_all().await;

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
) -> (Status, Json<models::common_model::Response<models::movie_model::GetMovieDetailResponse>>) {
    let mut response = models::common_model::Response{ 
        code: Status::Ok.code as u8, 
        status: Status::Ok.reason().unwrap().to_string(), 
        data: None, 
        errors: None,
    };

    let movie_repo = repository::movie_repository::new_movie_repository(&db_client.client);
    let movie_service = service::movie_service::new_movie_service(&movie_repo);
    let movie = movie_service.get_detail(id.to_string()).await;

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