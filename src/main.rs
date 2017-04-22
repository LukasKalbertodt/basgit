#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;


#[get("/")]
fn hello() -> &'static str {
    "Hello BasGit!"
}

fn main() {
    rocket::ignite()
        .mount("/", routes![hello])
        .launch();
}
