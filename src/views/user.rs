use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::users;

#[derive(Debug, Deserialize, Serialize)]
pub struct CurrentResponse {
    pub pid: String,
    pub username: String,
    pub email: String,
}

impl CurrentResponse {
    #[must_use]
    pub fn new(user: &users::Model) -> Self {
        Self {
            pid: user.pid.to_string(),
            username: user.username.clone(),
            email: user.email.clone(),
        }
    }
}

pub fn sets(
    v: &impl ViewRenderer,
    username: &str,
    sets: &Vec<crate::models::sets::Model>,
    own_user: &str,
) -> Result<Response> {
    format::render().view(
        v,
        "user/user.html",
        serde_json::json!({"page_user": username, "sets": sets, "username": own_user}),
    )
}
