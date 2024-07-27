use loco_rs::prelude::*;
use serde_json::json;

pub fn login(v: &impl ViewRenderer, user_name: &str) -> Result<impl IntoResponse> {
    format::render().view(v, "login.html", json!({"username": user_name}))
}
