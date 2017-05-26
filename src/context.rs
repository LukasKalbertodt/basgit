use rocket::request::FlashMessage;

use model::AuthUser;

/// Serves as the main template context.
#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct Context<T = ()> {
    /// Information about the user, if a login session exists.
    pub auth_user: Option<AuthUser>,

    /// Contents of the possibly set flash cookie.
    pub flash: Option<FlashContext>,

    /// A generic context for the main content.
    pub content: Option<T>,
}

impl<T> Default for Context<T> {
    fn default() -> Self {
        Context {
            auth_user: None,
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
            auth_user: None,
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

macro_rules! adhoc {
    ($($name:ident : $ty:ty = $val:expr,)+) => {{
        #[derive(Clone, Debug, Serialize)]
        struct Context<'a> {
            _dummy: ::std::marker::PhantomData<&'a ()>,
            $($name: $ty,)+
        }

        Context {
            _dummy: ::std::marker::PhantomData,
            $($name: $val,)+
        }
    }}
}
