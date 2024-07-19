use crate::models::users;
use loco_rs::prelude::*;

pub fn get_user_name(jwt_user: Option<auth::JWTWithUser<users::Model>>) -> String {
    jwt_user
        .map(|auth::JWTWithUser { claims: _, user }| user.name)
        .unwrap_or("".to_string())
}
