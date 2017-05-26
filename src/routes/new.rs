use rocket_contrib::Template;
use rocket::http::Status;
use rocket::response::Failure;

use model::AuthUser;

use context::Context;


///
#[get("/new")]
fn with_login(auth_user: AuthUser) -> Template {
    let context = Context {
        auth_user: Some(auth_user),
        .. Context::empty()
    };
    Template::render("new/with_login", &context)
}

#[get("/new", rank = 3)]
fn without_login() -> Failure {
    Failure(Status::Unauthorized)
}

// #[post("/new")]
// fn create(auth_user: AuthUser) -> Redirect {

// }
