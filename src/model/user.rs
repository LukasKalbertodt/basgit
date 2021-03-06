use diesel::prelude::*;
use diesel;
use hex;
use pwhash::bcrypt;
use rand::{self, Rng};
use rocket::{Outcome, State};
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request};
use std::ops::Deref;
use serde::{Serialize, Serializer};

use model::{self, Basket, BasketRecord, UserEmail, Session};
use model::permissions::{has_permission, UserAction};
use db::Db;
use db::schema::{baskets, users, user_emails, sessions};


const SESSION_COOKIE_NAME: &str = "session";
/// Length of the session id in bytes. 128 bit seems to be enough entropy
/// according to those sources:
///
/// - https://security.stackexchange.com/a/24852/147555
/// - https://security.stackexchange.com/a/138396/147555
///
/// If you change this value, you also have to change the database scheme,
/// since the length is checked there, too.
const SESSION_ID_LEN: usize = 16;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Identifiable, Queryable, Associations)]
#[has_many(user_emails)]
#[has_many(sessions)]
#[has_many(baskets)]
pub struct User {
    id: i64,

    /// May only contain ASCII alphanumeric characters and hyphens, but can't
    /// start with a hyphen.
    username: String,

    name: Option<String>,
    password: Option<String>,
    bio: Option<String>,
}

/// An authorized user with an active session. This type doesn't restrict
/// access to any properties, as the user is logged in.
#[derive(Clone, Eq, PartialEq)]
pub struct AuthUser {
    user: PubUser,
    session: Option<Session>,
}

impl AuthUser {
    /// Tries to authenticate a user with a given `id` (username or email) and
    /// a `password`. Returns a `User` object on success and an error
    /// otherwise.
    pub fn login(id: &str, password: &str, db: &Db) -> Result<Self, LoginError> {
        // TODO (whole method!): maybe avoid panic

        let conn = db.conn();

        // Usernames can't contain '@', so we can easily see whether or not
        // the `id` is an email address or a username.
        let user: Option<User> = if id.contains('@') {
            // Find the email in the database and return the user associated
            // with it.
            user_emails::table.find(id)
                .inner_join(users::table)
                .first(&*conn)
                .optional()
                .unwrap()
                .map(|(_, user): (UserEmail, User)| user)
        } else if is_valid_username(id) {
            // Find the user with the given username.
            users::table
                .filter(users::username.eq(id))
                .limit(1)
                .first(&*conn)
                .optional()
                .unwrap()
        } else {
            // If it's neither an email address nor a username, we don't need
            // to ask the database.
            None
        };

        user.ok_or(LoginError::UserNotFound).and_then(|user| {
            if user.password.is_none() {
                Err(LoginError::NoPasswordSet)
            } else if bcrypt::verify(password, user.password.as_ref().unwrap()) {
                Ok(AuthUser {
                    user: PubUser(user),
                    session: None,
                })
            } else {
                Err(LoginError::PasswordIncorrect)
            }
        })
    }

    pub fn into_pub_user(self) -> PubUser {
        self.user
    }

    pub fn create_session(&mut self, cookies: &Cookies, db: &Db) {
        // Generate a random session id.
        let mut id = [0u8; SESSION_ID_LEN];
        let mut rng = rand::os::OsRng::new()
            .expect("could not use system rng");
        rng.fill_bytes(&mut id);

        // Insert session id linked with the user id into the database.
        let new_session = model::NewSession {
            id: id.to_vec(),
            user_id: self.user.id(),
        };
        let inserted_session = diesel::insert(&new_session)
            .into(sessions::table)
            .get_result::<Session>(&*db.conn())
            .unwrap();
        self.session = Some(inserted_session);

        // Encode session id as hex and set it as cookie.
        let encoded = hex::encode(&id);
        cookies.add(Cookie::new("session", encoded));
    }

    /// Ends a login session, removing the entry from the database and removing
    /// the cookie.
    ///
    /// This function assumes the user was authenticated via session cookie.
    pub fn end_session(&self, cookies: &Cookies, db: &Db) {
        // Since we assume the user was authenticated via session id, we know
        // the cookie jar contains such a cookie and the cookie is a valid
        // hex string.
        let session_id = hex::decode(
            cookies.find(SESSION_COOKIE_NAME).unwrap().value()
        ).unwrap();

        // Remove from database.
        diesel::delete(sessions::table.find(session_id))
            .execute(&*db.conn())
            .expect("failed to delete session entry from database");

        // Remove from cookie jar.
        cookies.remove(SESSION_COOKIE_NAME);
    }
}

impl Deref for AuthUser {
    type Target = PubUser;
    fn deref(&self) -> &Self::Target {
        &self.user
    }
}

impl Serialize for AuthUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("auth_user", 4)?;
        s.serialize_field("id", &self.id())?;
        s.serialize_field("username", self.username())?;
        s.serialize_field("name", &self.name())?;
        s.serialize_field("bio", &self.bio())?;
        s.end()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // This method tries to authenticate a user from a session id.

        req.cookies().find(SESSION_COOKIE_NAME)
            // The cookie's value is encoded as hex string, but we need the
            // raw bytes.
            .and_then(|cookie| hex::decode(cookie.value()).ok())
            .and_then(|session_id| {
                // If the length is incorrect, we don't even need to ask the
                // database.
                if session_id.len() != SESSION_ID_LEN {
                    return None;
                }

                // Obtain a DB pool.
                let db = <State<Db> as FromRequest>::from_request(req)
                    .expect("cannot retrieve DB connection from request");

                // Try to find session id and the associated user.
                sessions::table
                    .find(session_id)
                    .inner_join(users::table)
                    .first::<(Session, User)>(&*db.conn())
                    .optional()
                    .unwrap()
            })
            // TODO: maybe check age of session
            .map(|(session, user)| {
                Outcome::Success(AuthUser {
                    user: PubUser(user),
                    session: Some(session),
                })
            })
            .unwrap_or(Outcome::Forward(()))
    }
}

pub enum LoginError {
    /// There is not user with the given id (email or username).
    UserNotFound,

    /// A user was found, but the given password is not correct.
    PasswordIncorrect,

    /// Login via password was attempted, but the user has no password set and
    /// can only authenticate with other methods.
    NoPasswordSet,
}

impl LoginError {
    pub fn description(&self) -> &'static str {
        match *self {
            LoginError::UserNotFound => "Username/email address not found.",
            LoginError::PasswordIncorrect => "Incorrect password.",
            LoginError::NoPasswordSet => "This user cannot be authenticated via password. \
                Please choose another authentication method.",
        }
    }
}

/// A public view of a user. Exposes only properties that are supposed to
/// be seen by everyone.
#[derive(Clone, Eq, PartialEq)]
pub struct PubUser(User);

impl PubUser {
    pub fn from_user(user: User) -> Self {
        PubUser(user)
    }

    pub fn from_username(username: &str, db: &Db) -> Option<Self> {
        // Quickly check whether the username has a correct format. If not,
        // we can reject it now already, without hitting the database.
        if !is_valid_username(username) {
            return None;
        }

        // Find the user with the given username.
        users::table
            .filter(users::username.eq(username))
            .limit(1)
            .first(&*db.conn())
            .optional()
            .unwrap()
            .map(PubUser)
    }

    pub fn id(&self) -> i64 {
        self.0.id
    }

    pub fn username(&self) -> &str {
        &self.0.username
    }

    pub fn name(&self) -> Option<&str> {
        self.0.name.as_ref().map(AsRef::as_ref)
    }

    pub fn bio(&self) -> Option<&str> {
        self.0.bio.as_ref().map(AsRef::as_ref)
    }

    pub fn baskets(&self, auth_user: Option<&AuthUser>, db: &Db) -> Vec<Basket> {
        BasketRecord::belonging_to(&self.0)
            .load(&*db.conn())
            .unwrap()
            .into_iter()
            .filter(|record| {
                has_permission(auth_user, UserAction::ViewBasket {
                    owner: self,
                    basket: record,
                })
            })
            .map(|record| Basket::from_parts(record, self.clone()))
            .collect()
    }
}

impl Serialize for PubUser {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("PubUser", 3)?;
        // Skipping id: the id should never be sent to the user
        s.serialize_field("username", self.username())?;
        s.serialize_field("name", &self.name())?;
        s.serialize_field("bio", &self.bio())?;
        s.end()
    }
}

fn is_valid_username(username: &str) -> bool {
    use std::ascii::AsciiExt;

    username.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !username.starts_with('-')
}
