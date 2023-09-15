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

impl TryInto<Movie> for &MovieJson {
    type Error = ();
    fn try_into(self) -> Result<Movie, Self::Error> {
        let movies_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "movies", "/");
        let image_path = Path::new(movies_dir).join(self.path.to_owned());
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
    let movies_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/", "movies");
    let manifest_path = Path::new(movies_dir).join("manifest.json");
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