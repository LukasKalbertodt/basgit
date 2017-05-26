use rocket_contrib::Template;
use rocket::http::Status;
use rocket::response::{Failure, Redirect, Flash};
use rocket::request::{Form, FlashMessage};
use rocket::State;

use context::Context;
use db::Db;
use model::{basket, AuthUser, UserAction};



///
#[get("/new")]
fn with_login(auth_user: AuthUser) -> Template {
    render_form(auth_user, None)
}


fn render_form(
    auth_user: AuthUser,
    error: Option<(String, Option<NewBasketForm>)>,
) -> Template {

    let (msg, form_data) = match error {
        Some(e) => (Some(e.0), Some(e.1)),
        None => (None, None),
    };
    let context = Context {
        flash: msg.map(|e| Flash::error((), e).into()),
        auth_user: Some(auth_user),
        content: form_data,
        .. Context::default()
    };
    Template::render("new/with_login", &context)
}

#[get("/new", rank = 3)]
fn without_login() -> Failure {
    Failure(Status::Unauthorized)
}



#[derive(Clone, Serialize, FromForm)]
struct NewBasketForm {
    owner: String,
    name: String,
    description: String,
    is_public: bool,
    kind: String,
}

#[post("/new", data = "<new_basket>")]
fn create(
    auth_user: AuthUser,
    new_basket: Option<Form<NewBasketForm>>,
    db: State<Db>,
) -> Result<Redirect, Template> {
    // Validate input data.
    {
        let new_basket = new_basket.map(|f| f.into_inner());
        let error = |msg| render_form(
            auth_user.clone(),
            Some((
                msg,
                new_basket.clone(),
            ))
        );

        let new_basket = new_basket.clone().ok_or_else(|| {
            error("Invalid form data!".to_string())
        })?;

        let action = UserAction::CreateBasket { owner: &new_basket.owner };
        if !auth_user.has_permission(action) {
            let msg = format!(
                "You don't have the permission to create a basket for '{}'!",
                new_basket.owner,
            );
            return Err(error(msg));
        }

        if new_basket.name.is_empty() {
            return Err(error("The basket's name can't be empty!".to_string()));
        }
        if !basket::is_valid_name(&new_basket.name) {
            return Err(error(
                "The basket's name contains invalid characters! Only \
                alphanumerical ASCII characters and dashes are allowed."
                    .to_string()
            ));

        }
    }

    Ok(Redirect::to("/"))
}
