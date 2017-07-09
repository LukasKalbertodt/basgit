use std::io::Cursor;

use serde_json;
use serde::Serialize;
use rocket::config::Environment;
use rocket::http::{ContentType, Status};
use rocket::response::{self, Response, Responder};

pub mod repo;

#[derive(Serialize, Deserialize)]
pub enum ApiResponse<T> {
    Ok(T),
    BadRequest {
        msg: String,
    },
    Unauthorized {
        msg: String,
    },
    NotFound,
    InternalServerError,
}


impl<'r, T: Serialize> Responder<'r> for ApiResponse<T> {
    fn respond(self) -> response::Result<'r> {
        let status = match self {
            ApiResponse::Ok(_) => Status::Ok,
            ApiResponse::BadRequest { .. } => Status::BadRequest,
            ApiResponse::Unauthorized { .. } => Status::Unauthorized,
            ApiResponse::NotFound => Status::NotFound,
            ApiResponse::InternalServerError => Status::InternalServerError,
        };

        fn to_json_string<T: Serialize>(obj: &T) -> String {
            let is_dev = Environment::active().unwrap() == Environment::Development;
            if is_dev {
                serde_json::to_string_pretty(obj).unwrap()
            } else {
                serde_json::to_string(obj).unwrap()
            }
        }

        let body = match self {
            ApiResponse::Ok(resp) => to_json_string(&resp),
            ApiResponse::BadRequest { msg } => to_json_string(&json!({ "msg": &msg })),
            ApiResponse::Unauthorized { msg } => to_json_string(&json!({ "msg": &msg })),
            ApiResponse::NotFound => "".to_string(),
            ApiResponse::InternalServerError => "".to_string(),
        };

        Response::build()
            .status(status)
            .sized_body(Cursor::new(body))
            .header(ContentType::JSON)
            .ok()
    }
}
