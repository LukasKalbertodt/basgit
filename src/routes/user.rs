use rocket_contrib::Template;
use rocket::State;
use rocket::request::{FormItems, FromForm};
use serde_json;

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

    PubUser::from_username(username, &db).map(|user| {
        let (template, key, value) = match tab {
            UserpageTab::Overview
                => overview_tab(&user, auth_user.as_ref(), &db),
            UserpageTab::Baskets
                => basket_tab(&user, auth_user.as_ref(), &db),
            UserpageTab::Stars
                => stars_tab(&user, auth_user.as_ref(), &db),
        };

        let user_url = format!("/{}", user.username());
        let title_name = match user.name() {
            Some(name) => format!("{} ({})", user.username(), name),
            None => user.username().to_string(),
        };

        let context = Context {
            auth_user,
            content: Some(json!({
                "user": user,
                "user_url": user_url,
                "title_name": title_name,
                key: value,
            })),
            .. Context::default()
        };

        Template::render(template, &context)
    })
}

fn overview_tab(
    _user: &PubUser,
    _auth_user: Option<&AuthUser>,
    _db: &Db,
) -> (&'static str, &'static str, serde_json::Value) {
    (
        "user/overview",
        "overview",
        json!({}),
    )
}

fn basket_tab(
    user: &PubUser,
    auth_user: Option<&AuthUser>,
    db: &Db,
) -> (&'static str, &'static str, serde_json::Value) {
    let baskets = user.baskets(auth_user, db);

    (
        "user/baskets",
        "baskets",
        json!(baskets),
    )
}

fn stars_tab(
    _user: &PubUser,
    _auth_user: Option<&AuthUser>,
    _db: &Db,
) -> (&'static str, &'static str, serde_json::Value) {
    (
        "user/stars",
        "stars",
        json!({}),
    )
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
