use chrono::DateTime;
use chrono::offset::utc::UTC;

use db::schema::sessions;
use user::User;

#[derive(Debug, Clone, Eq, PartialEq, Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub id: Vec<u8>,
    pub user_id: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Queryable, Associations)]
#[belongs_to(User)]
pub struct Session {
    pub id: Vec<u8>,
    pub user_id: i64,
    pub birth: DateTime<UTC>,
}
