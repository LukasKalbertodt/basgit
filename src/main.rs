#![feature(plugin, custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;


pub mod context;
pub mod db;
pub mod index;
pub mod login;
pub mod user;


fn main() {
    rocket::ignite()
        .manage(db::Db::open_connection())
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
