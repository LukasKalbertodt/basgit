#![feature(plugin, custom_derive)]
#![feature(ascii_ctype)]
#![plugin(rocket_codegen)]

extern crate base64;
extern crate chrono;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;
extern crate git2;
extern crate hex;
extern crate pwhash;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;


pub mod api;
pub mod context;
pub mod db;
pub mod model;
pub mod routes;

fn main() {
    use routes::{self, basket, index, login, new, user};
    use db::Db;

    rocket::ignite()
        .manage(Db::open_connection())
        .mount("/", routes![
            // Routes for serving the index page
            index::with_login,
            index::without_login,

            // All routes handling user log in and out
            login::with_login,
            login::without_login,
            login::validate_data,
            login::logout,

            // `/<user>` routes
            user::index,
            user::tabs,

            // `/new` for creating new baskets
            new::with_login,
            new::without_login,
            new::create,

            // All routes with the form `/<username>/<basket>`
            basket::index,
            basket::facade,

            // Serving static files in `static/`
            routes::static_files,
        ])
        .mount("/api/v1/", routes![
            api::v1::repo::tree,
            api::v1::repo::commit,
            api::v1::repo::blob,
            api::v1::repo::tree_entry,
        ])
        .launch();
}
