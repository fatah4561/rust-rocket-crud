use crate::models::movie_model;
use crate::repository::movie_repository::{MovieRepository, MovieRepositoryTrait};

pub struct MovieService<'a> {
    movie_repository: &'a MovieRepository<'a>,
}

pub fn new_movie_service<'a>(movie_repository: &'a MovieRepository) -> MovieService<'a> {
    MovieService { movie_repository }
}

#[async_trait]
pub trait MovieServiceTrait {
    async fn get_all(&self) -> Result<Vec<movie_model::GetAllMoviesResponse>, String>;
    async fn get_detail(&self, id: String) -> Result<movie_model::GetMovieDetailResponse, String>;
}

#[async_trait]
impl<'a> MovieServiceTrait for MovieService<'a> {
    async fn get_all(&self) -> Result<Vec<movie_model::GetAllMoviesResponse>, String> {
        let res = self.movie_repository.get_all().await;
        if let Err(ref e) = res {
            return Err(e.to_string());
        }

        let movies_db = res.unwrap();
        let mut movies = vec![];
        for movie_db in movies_db {
            let mut id = "".to_string();
            if movie_db.id.is_some() {
                id = movie_db.id.unwrap().to_hex()
            };

            let movie = movie_model::GetAllMoviesResponse {
                id,
                title: movie_db.title,
                year: movie_db.year as u32,
                released: movie_db.released.format("%d-%m-%Y %H:%M").to_string(),
            };
            movies.push(movie)
        }

        Ok(movies)
    }

    async fn get_detail(&self, id: String) -> Result<movie_model::GetMovieDetailResponse, String> {
        let movie = self
            .movie_repository
            .get_detail(id)
            .await
            .map_err(|e| e.to_string())
            .and_then(|movie| Ok(movie))?;

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
