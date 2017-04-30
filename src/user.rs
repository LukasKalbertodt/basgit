use rocket::{Outcome, State};
use rocket::request::{self, FromRequest, Request};

use db::Db;


#[derive(Clone, Eq, PartialEq, Serialize, Queryable)]
pub struct User {
    id: i64,
    username: String,
    name: Option<String>,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(req: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        use db::schema::users::dsl::*;
        use diesel::prelude::*;

        // TODO: this implementation is obviously ridiculous and intended for
        // testing only!

        req.cookies().find("username")
            .and_then(|cookie| {
                let db = <State<Db> as FromRequest>::from_request(req)
                    .expect("cannot retrieve DB connection from request");
                let conn = db.conn.lock().unwrap();

                // users.by_username();

                users
                    .filter(username.eq(cookie.value()))
                    .limit(1)
                    .first::<User>(&*conn)
                    .optional()
                    .expect("error loading users")
            })
            .map(|user| Outcome::Success(user))
            .unwrap_or(Outcome::Forward(()))
    }
}
