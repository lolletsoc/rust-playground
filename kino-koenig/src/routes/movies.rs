use std::path::Path;
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::get;
use rocket::serde::Serialize;
use rocket::State;
use crate::models::movie::Movie;

#[derive(Serialize)]
pub struct MovieResponse {
    name: String,
    released: u16,
    image_path: String,
}

impl From<&Movie> for MovieResponse {
    fn from(movie_ref: &Movie) -> Self {
        MovieResponse { name: movie_ref.name.to_owned(), released: movie_ref.released, image_path: movie_ref.image_path.to_owned() }
    }
}

#[get("/movies")]
pub fn movies(movies: &State<Vec<Movie>>) -> Json<Vec<MovieResponse>> {
    Json(movies.iter().map(|m| m.into()).collect())
}

#[get("/movies/<name>")]
pub async fn image(name: &str, movies: &State<Vec<Movie>>) -> Option<NamedFile> {
    match movies.iter().find(|m| m.name.eq_ignore_ascii_case(name)) {
        None => Option::None,
        Some(movie) => NamedFile::open(Path::new(crate::models::movie::MOVIES_DIR).join(movie.image_path.to_owned())).await.ok()
    }
}