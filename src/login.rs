use rocket_contrib::Template;
use rocket::response::{Flash, Redirect};
use rocket::request::{Form, FlashMessage};
use rocket::http::{Cookie, Cookies};

use user::User;

use context::Context;


/// The main login page showing a login form.
///
/// We might want to embed a smaller form into another route (the index page
/// for example), but this route will still be available.
#[get("/login", rank = 3)]
fn without_login(flash: Option<FlashMessage>) -> Template {
    let context = Context {
        flash: flash.map(|f| f.into()),
        .. Context::empty()
    };
    Template::render("login", &context)
}

/// Handler in case the `/login` page is access although the user is already
/// logged in. We just redirect to the index page.
#[get("/login")]
fn with_login(_user: User) -> Redirect {
    // TODO: GitHub uses the 302 status code to redirect, but the `to()` method
    // uses the code 303. The rocket docs say 303 is preferred over 302, but
    // we should look for more information on this.
    Redirect::to("/")
}

/// Handles post data from a login action.
#[post("/login", data = "<form_data>")]
fn validate_data(
    cookies: &Cookies,
    form_data: Form<LoginForm>
) -> Result<Redirect, Flash<Redirect>> {
    // TODO: Again, this is obviously just for testing and should be replaced
    // by a real login system.
    let user_name = form_data.into_inner().user_name;
    if user_name != "invalid" {
        cookies.add(Cookie::new("user_name", user_name));
        Ok(Redirect::to("/"))
    } else {
        Err(Flash::error(Redirect::to("/login"), "Invalid login data"))
    }
}

/// Handler to logout the user. If there is no login present, nothing happens.
#[get("/logout")]
fn logout(cookies: &Cookies, user: Option<User>) -> Redirect {
    if let Some(_) = user {
        // TODO: replace by proper login system
        cookies.remove("user_name");
    }
    Redirect::to("/")
}

#[derive(FromForm)]
struct LoginForm {
    user_name: String,
}
