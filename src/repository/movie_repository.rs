use std::sync::Arc;

use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    error::Error,
    Client,
};

use crate::entity::movie_entity;
use crate::exception::error::CustomError;
use crate::rocket::futures::TryStreamExt; // for try_next() trait (don't remove)

pub struct MovieRepository {
    mongo: Arc<Client>,
}

#[async_trait]
pub trait MovieRepositoryTrait {
    async fn get_all(&self) -> Result<Vec<movie_entity::Movie>, CustomError>;
    async fn get_detail(&self, id: String) -> Result<movie_entity::Movie, CustomError>;
    fn insert(&self) -> Result<(), Error>;
    fn update(&self, request: movie_entity::Movie) -> Result<(), Error>;
    fn delete(&self, id: String) -> Result<(), Error>;
}

pub fn new_movie_repository(mongo: Arc<Client>) -> MovieRepository {
    MovieRepository { mongo }
}

#[async_trait]
impl MovieRepositoryTrait for MovieRepository {
    async fn get_all(&self) -> Result<Vec<movie_entity::Movie>, CustomError> {
        let collection: mongodb::Collection<Document> =
            self.mongo.database("sample_mflix").collection("movies");
        let mut movie_cursor = collection
            .find(doc! {}, None)
            .await
            .map_err(|e| CustomError::internal_server_error(e.to_string()))?;

        let mut movies = vec![];
        while let Ok(Some(doc)) = movie_cursor.try_next().await {
            let deserialized_movie: Result<movie_entity::Movie, bson::de::Error> =
                bson::from_bson(Bson::Document(doc));
            if let Err(ref e) = deserialized_movie {
                return Err(CustomError::internal_server_error(e.to_string()));
            }
            movies.push(deserialized_movie.unwrap());
        }

        Ok(movies)
    }

    async fn get_detail(&self, id: String) -> Result<movie_entity::Movie, CustomError> {
        let collection: mongodb::Collection<Document> =
            self.mongo.database("sample_mflix").collection("movies");

        let movie_obj_id = match ObjectId::parse_str(&id) {
            Ok(id) => id,
            Err(_) => {
                return Err(CustomError::bad_request_error(
                    "format id invalid".to_string(),
                    Some("id".to_string()),
                ));
            }
        };

        match collection
            .find_one(doc! { "_id": movie_obj_id }, None)
            .await
        {
            Ok(Some(movie)) => {
                let deserialized_movie: movie_entity::Movie =
                    match bson::from_bson(Bson::Document(movie)) {
                        Ok(movie) => movie,
                        Err(e) => {
                            return Err(CustomError::internal_server_error(e.to_string()));
                        }
                    };

                return Ok(deserialized_movie);
            }
            Ok(None) => {
                return Err(CustomError::not_found_error("Data not found".to_string()));
            }
            Err(e) => {
                return Err(CustomError::internal_server_error(e.to_string()));
            }
        }
    }

    fn insert(&self) -> Result<(), Error> {
        todo!()
    }

    fn update(&self, request: movie_entity::Movie) -> Result<(), Error> {
        todo!()
    }

    fn delete(&self, id: String) -> Result<(), Error> {
        todo!()
    }
}
