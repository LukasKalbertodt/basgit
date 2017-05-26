use rocket_contrib::Template;
use rocket::State;

use model::{AuthUser, Basket};
use context::Context;
use db::Db;


#[get("/<username>/<basket>", rank = 10)]
pub fn index(
    username: &str,
    basket: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
) -> Option<Template> {
    handler(username, basket, auth_user, db, None)
}

fn handler(
    username: &str,
    basket: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
    _facade: Option<&str>,
) -> Option<Template> {
    Basket::load(basket, username, auth_user.as_ref(), &db)
        .map(|basket| {
            // TODO: load facade

            #[derive(Debug, Clone, Serialize)]
            struct BasketHeaderContext<'a> {
                owner: &'a str,
                name: &'a str,
                description: Option<&'a str>,
            }

            let context = Context {
                auth_user,
                content: Some(BasketHeaderContext {
                    owner: basket.owner(),
                    name: basket.name(),
                    description: basket.description(),
                }),
                .. Context::default()
            };

            let template = "basket/settings";
            Template::render(template, &context)
        })
}
