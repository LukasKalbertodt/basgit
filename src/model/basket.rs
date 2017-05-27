use diesel::prelude::*;
use diesel;
use serde::{Serialize, Serializer};
use std::fmt;
use std::ops::Deref;

use db::schema::baskets;
use db::schema::users;

use db::Db;
use model::{basket, AuthUser, PubUser, User};
use model::permissions::{has_permission, UserAction};
use routes::new::NewBasketForm;
use super::MAX_SL_LEN;



pub fn is_valid_name(s: &str) -> bool {
    use std::ascii::AsciiExt;

    s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
        && !s.starts_with('-')
        && s.len() < MAX_SL_LEN
}


#[derive(Clone, Debug, Serialize, Identifiable, Queryable, Associations)]
#[table_name = "baskets"]
#[belongs_to(User)]
pub struct BasketRecord {
    id: i64,
    name: String,
    user_id: i64,
    description: Option<String>,
    public: bool,
    kind: String,
    forked_from: Option<i64>,
}

impl BasketRecord {
    pub fn is_public(&self) -> bool {
        self.public
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(AsRef::as_ref)
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }
}

#[derive(Clone, Debug, Insertable)]
#[table_name = "baskets"]
pub struct NewBasket {
    name: String,
    user_id: i64,
    description: Option<String>,
    public: bool,
    kind: String,
    forked_from: Option<i64>,
}

pub struct Basket {
    record: BasketRecord,
    user: PubUser,
}

impl Basket {
    pub fn from_parts(record: BasketRecord, user: PubUser) -> Self {
        Self { record, user }
    }
    pub fn create(
        new: NewBasketForm,
        auth_user: &AuthUser,
        db: &Db
    ) -> Result<Self, CreateError> {
        use diesel::result::{Error as DieselError, DatabaseErrorKind};

        if !has_permission(Some(auth_user), UserAction::CreateBasket { owner: &new.owner }) {
            return Err(CreateError::NoPermission { owner: new.owner });
        }

        if new.name.is_empty() {
            return Err(CreateError::NameEmpty);
        }
        if !basket::is_valid_name(&new.name) {
            return Err(CreateError::NameInvalid);
        }

        // TODO: in case we introduce organizations, this need to change.
        // We can unwrap, because we checked above, whether the current user
        // can create baskets for the given owner. It should have returned
        // "false" if the owner doesn't even exist.
        let user = PubUser::from_username(&new.owner, db).unwrap();

        let description = if new.description.trim().is_empty() {
            None
        } else {
            Some(new.description.trim().into())
        };

        let new_basket = NewBasket {
            name: new.name,
            user_id: user.id(),
            description: description,
            public: new.is_public,
            kind: new.kind,
            forked_from: None,
        };

        let inserted = diesel::insert(&new_basket)
            .into(baskets::table)
            .get_result::<BasketRecord>(&*db.conn());

        if let Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) = inserted {
            return Err(CreateError::NameAlreadyUsed);
        }

        Ok(Self {
            record: inserted.unwrap(),
            user,
        })
    }

    pub fn load(
        name: &str,
        owner: &str,
        auth_user: Option<&AuthUser>,
        db: &Db,
    ) -> Option<Self> {
        baskets::table
            .inner_join(users::table)
            .filter(baskets::name.eq(name))
            .filter(users::username.eq(owner))
            .first(&*db.conn())
            .optional()
            .unwrap()
            // .map(|(data, user)| Self { data, user: PubUser::from_user(user) })
            .and_then(|(record, user)| {
                let user = PubUser::from_user(user);
                let can_view = has_permission(auth_user, UserAction::ViewBasket {
                    owner: &user,
                    basket: &record,
                });
                if can_view {
                    Some(Self { record, user })
                } else {
                    None
                }
            })
    }

    pub fn owner(&self) -> &str {
        self.user.username()
    }

    pub fn url(&self) -> String {
        format!("/{}/{}", self.user.username(), self.record.name)
    }
}

impl Deref for Basket {
    type Target = BasketRecord;
    fn deref(&self) -> &Self::Target {
        &self.record
    }
}

impl Serialize for Basket {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        use serde::ser::SerializeStruct;

        let mut s = serializer.serialize_struct("Basket", 6)?;
        // Skipping id: the id should never be sent to the user
        s.serialize_field("name", self.name())?;
        s.serialize_field("description", &self.description())?;
        s.serialize_field("is_public", &self.is_public())?;
        s.serialize_field("url", &self.url())?;
        s.serialize_field("kind", self.kind())?;
        s.serialize_field("owner", self.owner())?;
        s.end()
    }
}

pub enum CreateError {
    /// The current user does not have the permission to create a basket for
    /// the given owner.
    NoPermission {
        owner: String,
    },
    NameEmpty,
    NameInvalid,
    NameAlreadyUsed,
}

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CreateError::*;

        match *self {
            NoPermission { ref owner } => {
                write!(
                    f,
                    "You don't have the permission to create a basket for '{}'!",
                    owner,
                )
            }
            NameEmpty => {
                "The basket's name can't be empty!".fmt(f)
            }
            NameInvalid => {
                "The basket's name contains invalid characters! Only \
                alphanumerical ASCII characters and dashes are allowed."
                    .fmt(f)
            }
            NameAlreadyUsed => {
                "A repository with the given name already exists for the \
                given owner"
                    .fmt(f)
            }
        }
    }
}
