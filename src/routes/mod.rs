use rocket::response::NamedFile;


pub mod index;
pub mod login;


/// Route to serve static file requests from the `static/` directory.
#[get("/static/<file>")]
pub fn static_files(file: &str) -> Option<NamedFile> {
    use std::path::Path;

    NamedFile::open(Path::new("static/").join(file)).ok()
}
