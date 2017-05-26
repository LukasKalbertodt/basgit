use model::User;

use db::schema::user_emails;

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Identifiable, Queryable, Associations)]
#[primary_key(email)]
#[belongs_to(User)]
pub struct UserEmail {
    email: String,
    user_id: i64,
}
