use crate::models::users;
use loco_rs::prelude::*;

#[must_use]
pub fn get_user_name(jwt_user: Option<auth::JWTWithUser<users::Model>>) -> Option<String> {
    jwt_user.map(|jwt_user| jwt_user.user.name)
}
