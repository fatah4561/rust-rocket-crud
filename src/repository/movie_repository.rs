use mongodb::{
    bson::{doc, oid::ObjectId, Bson, Document},
    error::Error,
    Client,
};

use crate::entity::movie_entity;
use crate::rocket::futures::TryStreamExt; // for try_next() trait

pub struct MovieRepository<'a> {
    mongo: &'a Client,
}

#[async_trait]
pub trait MovieRepositoryTrait {
    async fn get_all(&self) -> Result<Vec<movie_entity::Movie>, Error>;
    async fn get_detail(&self, id: String) -> Result<movie_entity::Movie, Error>;
    fn insert(&self) -> Result<(), Error>;
    fn update(&self, request: movie_entity::Movie) -> Result<(), Error>;
    fn delete(&self, id: String) -> Result<(), Error>;
}

pub fn new_movie_repository(client: &Client) -> MovieRepository {
    MovieRepository { mongo: client }
}

#[async_trait]
impl<'a> MovieRepositoryTrait for MovieRepository<'a> {
    async fn get_all(&self) -> Result<Vec<movie_entity::Movie>, Error> {
        let collection: mongodb::Collection<Document> =
            self.mongo.database("sample_mflix").collection("movies");
        let mut movie_cursor = collection.find(doc! {}, None).await?;

        let mut movies = vec![];
        while let Ok(Some(doc)) = movie_cursor.try_next().await {
            let deserialized_movie: Result<movie_entity::Movie, bson::de::Error> =
                bson::from_bson(Bson::Document(doc));
            if let Err(ref e) = deserialized_movie {
                return Err(mongodb::error::Error::from(e.clone()));
            }
            movies.push(deserialized_movie.unwrap());
        }

        Ok(movies)
    }

    async fn get_detail(&self, id: String) -> Result<movie_entity::Movie, Error> {
        let collection: mongodb::Collection<Document> =
            self.mongo.database("sample_mflix").collection("movies");

        let movie_obj_id = match ObjectId::parse_str(&id) {
            Ok(id) => id,
            Err(e) => {
                return Err(Error::from(std::io::Error::new(
                    std::io::ErrorKind::InvalidInput,
                    format!("Invalid ObjectId: {}", e),
                )));
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
                            return Err(e.into());
                        }
                    };

                return Ok(deserialized_movie);
            }
            Ok(None) => {
                return Err(Error::from(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Not Found for id: {id}"),
                )));
            }
            Err(e) => return Err(e)

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
