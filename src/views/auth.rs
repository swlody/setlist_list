use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::models::_entities::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginResponse {
    pub pid: String,
    pub name: String,
    pub is_verified: bool,
}

impl LoginResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            name: user.name.clone(),
            is_verified: user.email_verified_at.is_some(),
        }
    }
}

pub fn login(v: &impl ViewRenderer, user_name: &str) -> Result<impl IntoResponse> {
    format::render().view(v, "login.html", json!({"username": user_name}))
}
