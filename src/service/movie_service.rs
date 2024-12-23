use crate::exception::error::CustomError;
use crate::models::movie_model;
use crate::repository::movie_repository::{MovieRepository, MovieRepositoryTrait};

use std::sync::Arc;

pub struct MovieService {
    movie_repository: Arc<MovieRepository>,
}

pub fn new_movie_service(movie_repository: Arc<MovieRepository>) -> MovieService {
    MovieService { movie_repository }
}

#[async_trait]
pub trait MovieServiceTrait {
    async fn get_all(&self) -> Result<Vec<movie_model::GetAllMoviesResponse>, CustomError>;
    async fn get_detail(
        &self,
        id: String,
    ) -> Result<movie_model::GetMovieDetailResponse, CustomError>;
}

#[async_trait]
impl<'a> MovieServiceTrait for MovieService {
    async fn get_all(&self) -> Result<Vec<movie_model::GetAllMoviesResponse>, CustomError> {
        let res = self.movie_repository.get_all().await?;

        let mut movies = vec![];
        for movie in res {
            let mut id = "".to_string();
            if movie.id.is_some() {
                id = movie.id.unwrap().to_hex()
            };

            movies.push(movie_model::GetAllMoviesResponse {
                id,
                title: movie.title,
                year: movie.year as u32,
                released: movie.released.format("%d-%m-%Y %H:%M").to_string(),
            })
        }

        Ok(movies)
    }

    async fn get_detail(
        &self,
        id: String,
    ) -> Result<movie_model::GetMovieDetailResponse, CustomError> {
        let movie = self.movie_repository.get_detail(id).await?;

        let mut id = "".to_string();
        if let Some(movie_id) = &movie.id {
            id = movie_id.to_hex()
        };

        Ok(movie_model::GetMovieDetailResponse {
            id,
            title: movie.title,
            year: movie.year as u32,
            plot: movie.plot,
            released: movie.released.format("%d-%m-%Y %H:%M").to_string(),
        })
    }
}
