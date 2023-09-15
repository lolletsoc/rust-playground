use std::fs::File;
use std::path::Path;

use serde::{Deserialize};

#[derive(Deserialize)]
struct MoviesManifest {
    movies: Vec<MovieJson>,
}

#[derive(Deserialize)]
struct MovieJson {
    name: String,
    released: u16,
    path: String,
}

const MOVIES_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/", "movies");

impl TryInto<Movie> for &MovieJson {
    type Error = ();
    fn try_into(self) -> Result<Movie, Self::Error> {
        let image_path = Path::new(MOVIES_DIR).join(self.path.to_owned());
        match File::open(image_path) {
            Ok(file) => Ok(Movie { name: self.name.to_owned(), image: file }),
            Err(..) => Err(())
        }
    }
}

pub struct Movie {
    pub name: String,
    pub image: File,
}

pub fn load_movies() -> Vec<Movie> {
    let manifest_path = Path::new(MOVIES_DIR).join("manifest.json");
    match File::open(manifest_path) {
        Ok(file) => {
            let movie_json: MoviesManifest = serde_json::from_reader(file).unwrap();
            movie_json.movies.iter()
                .map(|mj| mj.try_into())
                .filter(|r| r.is_ok())
                .map(|r| r.unwrap())
                .collect()
        }
        Err(_) => vec![]
    }
}