use rocket_contrib::Template;

use model::AuthUser;

use context::Context;


/// The landing page in case there is a valid user login.
///
/// The plan is to show recent activity in watched baskets as well as some
/// quick action buttons, such as "create new basket".
#[get("/")]
fn with_login(auth_user: AuthUser) -> Template {
    let context = Context {
        auth_user: Some(auth_user),
        .. Context::empty()
    };
    Template::render("index/with_login", &context)
}

/// The landing page without a valid login (for new users).
///
/// Here we typically show some good arguments why the visitor should use our
/// site as well as a friendly login box.
#[get("/", rank = 3)]
fn without_login() -> Template {
    Template::render("index/without_login", &Context::empty())
}
