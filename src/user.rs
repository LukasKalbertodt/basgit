use rocket::Outcome;
use rocket::request::{self, FromRequest, Request};


#[derive(Clone, Eq, PartialEq, Serialize)]
pub struct User {
    name: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for User {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        // TODO: this implementation is obviously ridiculous and intended for
        // testing only!

        match request.cookies().find("user_name") {
            Some(cookie) => Outcome::Success(Self { name: cookie.value().into() }),
            None => Outcome::Forward(()),
        }
    }
}
