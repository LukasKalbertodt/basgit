use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use std::sync::Mutex;

pub mod schema;

pub struct Db {
    // TODO: Wrapping the connection into a mutex is probably a pretty bad
    // idea regarding performance. Also, [here][1] it says that connections
    // should only be used on a single thread anyway. So I guess this should
    // be replaced by a mechanism which lazily opens a connection for every
    // thread. Something like that.
    //
    // [1]: https://github.com/diesel-rs/diesel/issues/190
    pub conn: Mutex<PgConnection>,
}

impl Db {
    pub fn open_connection() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        let conn = PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url));

        Self {
            conn: Mutex::new(conn),
        }
    }
}
