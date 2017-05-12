use rocket_contrib::Template;
// use rocket::response::{Flash, Redirect};
use rocket::State;

use model::{AuthUser, PubUser};
use context::Context;
use db::Db;

#[get("/<username>", rank = 10)]
pub fn index(username: &str, auth_user: Option<AuthUser>, db: State<Db>) -> Option<Template> {
    #[derive(Debug, Clone, Serialize)]
    struct UserIndexContext<'a> {
        username: &'a str,
        name: Option<&'a str>,
        bio: Option<&'a str>,
    }


    PubUser::from_username(username, &db)
        .map(|pub_user| {
            let context = Context {
                auth_user,
                content: Some(UserIndexContext {
                    username: pub_user.username(),
                    name: pub_user.name(),
                    bio: pub_user.bio(),
                }),
                .. Context::default()
            };

            Template::render("user/index", &context)
        })
}
