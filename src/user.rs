use diesel::prelude::*;
use pwhash::bcrypt;
use rocket::{Outcome, State};
use rocket::http::{Cookie, Cookies};
use rocket::request::{self, FromRequest, Request};

use db::Db;

use db::schema::{users, user_emails};


#[derive(Debug, Clone, Eq, PartialEq, Serialize, Identifiable, Queryable, Associations)]
#[has_many(user_emails)]
pub struct User {
    id: i64,

    /// May only contain ASCII alphanumeric characters and hyphens, but can't
    /// start with a hyphen.
    username: String,

    name: Option<String>,
    password: Option<String>,
    bio: Option<String>,
}

impl User {
    /// Tries to authenticate a user with a given `id` (username or email) and
    /// a `password`. Returns a `User` object on success and an error
    /// otherwise.
    pub fn login(id: &str, password: &str, db: &Db) -> Result<Self, LoginError> {
        // TODO (whole method!): maybe avoid panic

        let conn = db.conn();

        // Usernames can't contain '@', so we can easily see whether or not
        // the `id` is an email address or a username.
        let user: Option<Self> = if id.contains('@') {
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
                    Ok(user)
                } else {
                    Err(LoginError::PasswordIncorrect)
                }
            }
            None => Err(LoginError::UserNotFound),
        }
    }

    pub fn set_session(&self, cookies: &Cookies) {
        // TODO: this is obviously stupid
        cookies.add(Cookie::new("username", self.username.clone()));
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // TODO: this implementation is obviously ridiculous and intended for
        // testing only!

        req.cookies().find("username")
            .and_then(|cookie| {
                let db = <State<Db> as FromRequest>::from_request(req)
                    .expect("cannot retrieve DB connection from request");
                let conn = db.pool.get().unwrap();

                users::table
                    .filter(users::username.eq(cookie.value()))
                    .limit(1)
                    .first::<User>(&*conn)
                    .optional()
                    .expect("error loading users")
            })
            .map(|user| Outcome::Success(user))
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

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Identifiable, Queryable, Associations)]
#[primary_key(email)]
#[belongs_to(User)]
pub struct UserEmail {
    email: String,
    user_id: i64,
}
