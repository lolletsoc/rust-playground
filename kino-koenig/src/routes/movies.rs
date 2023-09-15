use rocket::get;
use rocket::State;
use crate::models::movie::Movie;

#[get("/")]
pub fn index(movies: &State<Vec<Movie>>) -> String {
    movies.len().to_string()
}
