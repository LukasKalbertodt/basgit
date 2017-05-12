use base64;
use diesel::prelude::*;
use diesel;
use pwhash::bcrypt;
use rand::{self, Rng};
use rocket::{Outcome, State};
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request};
use model::{self, UserEmail, Session};


use db::Db;

use db::schema::{users, user_emails, sessions};


const SESSION_COOKIE_NAME: &str = "session";

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Identifiable, Queryable, Associations)]
#[has_many(user_emails)]
#[has_many(sessions)]
pub struct User {
    id: i64,

    /// May only contain ASCII alphanumeric characters and hyphens, but can't
    /// start with a hyphen.
    username: String,

    name: Option<String>,
    password: Option<String>,
    bio: Option<String>,
}

pub struct AuthUser {
    data: User,
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

        } else {
            // Find the user with the given username.
            users::table
                .filter(users::username.eq(id))
                .limit(1)
                .first(&*conn)
                .optional()
                .unwrap()
        };

        match user {
            Some(user) => {
                if user.password.is_none() {
                    Err(LoginError::NoPasswordSet)
                } else if bcrypt::verify(password, user.password.as_ref().unwrap()) {
                    Ok(AuthUser {
                        data: user,
                        session: None,
                    })
                } else {
                    Err(LoginError::PasswordIncorrect)
                }
            }
            None => Err(LoginError::UserNotFound),
        }
    }

    pub fn into_data(self) -> User {
        self.data
    }

    pub fn create_session(&mut self, cookies: &Cookies, db: &Db) {
        // Generate a random session id. 128 bit seems to be enough entropy
        // according to those sources:
        //
        // - https://security.stackexchange.com/a/24852/147555
        // - https://security.stackexchange.com/a/138396/147555
        let mut id = [0u8; 16];
        let mut rng = rand::os::OsRng::new()
            .expect("could not use system rng");
        rng.fill_bytes(&mut id);

        // Insert session id linked with the user id into the database.
        let new_session = model::NewSession {
            id: id.to_vec(),
            user_id: self.data.id,
        };
        let inserted_session = diesel::insert(&new_session)
            .into(sessions::table)
            .get_result::<Session>(&*db.conn())
            .unwrap();
        self.session = Some(inserted_session);

        // Encode session id as base64 and set it as cookie.
        let encoded = base64::encode(&id);
        cookies.add(Cookie::new("session", encoded));
    }

    /// Ends a login session, removing the entry from the database and removing
    /// the cookie.
    ///
    /// This function assumes the user was authenticated via session cookie.
    pub fn end_session(&self, cookies: &Cookies, db: &Db) {
        // Since we assume the user was authenticated via session id, we know
        // the cookie jar contains such a cookie and the cookie is valid
        // base64.
        let session_id = base64::decode(
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

impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // This method tries to authenticate a user from a session id.

        req.cookies().find(SESSION_COOKIE_NAME)
            // The cookie's value is encoded in base64, but we need the raw
            // bytes.
            .and_then(|cookie| base64::decode(cookie.value()).ok())
            .and_then(|session_id| {
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
                    data:user,
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
