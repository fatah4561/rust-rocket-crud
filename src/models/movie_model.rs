use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAllMoviesResponse {
    pub id: String,
    pub title: String,
    pub year: u32,
    pub released: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetMovieDetailResponse {
    pub id: String,
    pub title: String,
    pub year: u32,
    pub plot: String,
    pub released: String,
}
