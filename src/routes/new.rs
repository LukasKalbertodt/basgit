use rocket_contrib::Template;
use rocket::http::Status;
use rocket::response::{Failure, Redirect, Flash};
use rocket::request::Form;
use rocket::State;

use context::Context;
use db::Db;
use model::{AuthUser, Basket};



///
#[get("/new")]
fn with_login(auth_user: AuthUser) -> Template {
    render_form(auth_user, None, None)
}


fn render_form(
    auth_user: AuthUser,
    error: Option<String>,
    values: Option<NewBasketForm>,
) -> Template {

    let context = Context {
        flash: error.map(|e| Flash::error((), e).into()),
        auth_user: Some(auth_user),
        content: values,
        .. Context::default()
    };
    Template::render("new/with_login", &context)
}

#[get("/new", rank = 3)]
fn without_login() -> Failure {
    Failure(Status::Unauthorized)
}



#[derive(Clone, Serialize, FromForm)]
pub struct NewBasketForm {
    pub owner: String,
    pub name: String,
    pub description: String,
    pub is_public: bool,
    pub kind: String,
}

#[post("/new", data = "<new>")]
fn create(
    auth_user: AuthUser,
    new: Option<Form<NewBasketForm>>,
    db: State<Db>,
) -> Result<Redirect, Template> {
    // Check if the post request contains the correct data that should be
    // saved in the `NewBasketForm`.
    let new = match new {
        Some(form) => form.into_inner(),
        None => {
            return Err(render_form(
                auth_user,
                Some("Invalid form data!".into()),
                None
            ));
        }
    };

    let form_data_clone = new.clone();
    let basket = Basket::create(new, &auth_user, &db)
        .map_err(|e| {
            render_form(
                auth_user,
                Some(e.to_string()),
                Some(form_data_clone),
            )
        })?;

    Ok(Redirect::to(&basket.url()))
}
