#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

mod context;
mod index;
mod login;
mod user;


fn main() {
    rocket::ignite()
        .mount("/", routes![
            // Routes for serving the index page
            index::with_login,
            index::without_login,

            // All routes handling user log in and out
            login::with_login,
            login::without_login,
            login::validate_data,
            login::logout,

            // Serving static files in `static/`
            static_files,
        ])
        .launch();
}



use rocket::response::NamedFile;

/// Route to serve static file requests from the `static/` directory.
#[get("/static/<file>")]
fn static_files(file: &str) -> Option<NamedFile> {
    use std::path::Path;

    NamedFile::open(Path::new("static/").join(file)).ok()
}
