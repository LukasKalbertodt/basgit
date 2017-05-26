use rocket_contrib::Template;
use rocket::State;
use rocket::request::{FormItems, FromForm};

use model::{AuthUser, PubUser};
use context::Context;
use db::Db;


#[get("/<username>", rank = 10)]
pub fn index(username: &str, auth_user: Option<AuthUser>, db: State<Db>) -> Option<Template> {
    handler(username, auth_user, db, UserpageTab::Overview)
}

#[get("/<username>?<tab>", rank = 10)]
pub fn tabs(
    username: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
    tab: UserpageTab,
) -> Option<Template> {
    handler(username, auth_user, db, tab)
}

fn handler(
    username: &str,
    auth_user: Option<AuthUser>,
    db: State<Db>,
    tab: UserpageTab,
) -> Option<Template> {
    #[derive(Debug, Clone, Serialize)]
    struct UserIndexContext<'a> {
        username: &'a str,
        name: Option<&'a str>,
        bio: Option<&'a str>,
    }

    PubUser::from_username(username, &db).map(|pub_user| {
        let context = Context {
            auth_user,
            content: Some(UserIndexContext {
                username: pub_user.username(),
                name: pub_user.name(),
                bio: pub_user.bio(),
            }),
            .. Context::default()
        };

        let template = match tab {
            UserpageTab::Overview => "user/overview",
            UserpageTab::Baskets => "user/baskets",
            UserpageTab::Stars => "user/stars",
        };

        Template::render(template, &context)
    })
}

pub enum UserpageTab {
    Overview,
    Baskets,
    Stars,
}

impl<'f> FromForm<'f> for UserpageTab {
    type Error = ();
    fn from_form_items(form_items: &mut FormItems<'f>) -> Result<Self, Self::Error> {
        let out = form_items
            .find(|&(key, _)| key == "tab")
            .and_then(|(_, value)| {
                match value {
                    "baskets" => Some(UserpageTab::Baskets),
                    "stars" => Some(UserpageTab::Stars),
                    _ => None,
                }
            })
            .unwrap_or(UserpageTab::Overview);
        Ok(out)
    }
}
