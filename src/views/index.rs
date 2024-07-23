use loco_rs::prelude::*;
use serde_json::json;

pub fn root(v: &impl ViewRenderer, user_name: &str) -> Result<impl IntoResponse> {
    format::render().view(v, "index.html", json!({"username": user_name}))
}

pub fn not_found(v: &impl ViewRenderer, user_name: &str) -> Result<Response> {
    format::render()
        .status(axum::http::StatusCode::NOT_FOUND)
        .view(
            v,
            "404.html",
            json!({"some": "value", "username": user_name}),
        )
}
