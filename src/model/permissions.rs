use model::{AuthUser, BasketRecord, PubUser};


pub enum UserAction<'a> {
    /// Creating a new basket for a given owner.
    CreateBasket {
        owner: &'a str,
    },
    ViewBasket {
        owner: &'a PubUser,
        basket: &'a BasketRecord,
    },
}

pub fn has_permission(user: Option<&AuthUser>, action: UserAction) -> bool {
    use self::UserAction::*;

    match action {
        CreateBasket { owner } => {
            // TODO: this will change in the far future
            user.map(|u| owner == u.username()).unwrap_or(false)
        }
        ViewBasket { owner, basket } => {
            basket.is_public() ||
                user.map(|u| owner.username() == u.username()).unwrap_or(false)
        }
    }
}
