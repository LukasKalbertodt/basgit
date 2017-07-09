use std::path::{Path, PathBuf};

use rocket::response::NamedFile;


pub mod basket;
pub mod index;
pub mod login;
pub mod new;
pub mod user;


/// Route to serve static file requests from the `static/` directory.
#[get("/static/<path..>")]
pub fn static_files(path: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(path)).ok()
}
