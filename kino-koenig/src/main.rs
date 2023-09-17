#[macro_use]
extern crate rocket;

use kino::models::movie::load_movies;
use kino::routes::movies::{image, movies};

#[launch]
fn rocket() -> _ {
    let instance = rocket::build().mount("/", routes![movies, image]);
    let movies = load_movies();
    instance.manage(movies)
}
