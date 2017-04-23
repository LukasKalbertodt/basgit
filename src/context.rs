use rocket::request::FlashMessage;

use user::User;

/// Serves as the main template context.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct Context<T = ()> {
    /// Information about the user, if a login session exists.
    pub user: Option<User>,

    /// Contents of the possibly set flash cookie.
    pub flash: Option<FlashContext>,

    /// A generic context for the main content.
    pub content: Option<T>,
}

impl<T> Default for Context<T> {
    fn default() -> Self {
        Context {
            user: None,
            flash: None,
            content: None,
        }
    }
}

impl Context<()> {
    /// Just a helper function which fixes the type parameter to `()`. Without
    /// this, we would need to write `Context::<()>::default()` everywhere.
    pub fn empty() -> Context<()> {
        Context {
            user: None,
            flash: None,
            content: None,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct FlashContext {
    pub name: String,
    pub msg: String,
}

impl From<FlashMessage> for FlashContext {
    fn from(fm: FlashMessage) -> Self {
        Self {
            name: fm.name().into(),
            msg: fm.msg().into(),
        }
    }
}
