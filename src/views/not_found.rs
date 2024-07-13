use loco_rs::prelude::*;
use serde_json::json;

pub fn not_found(v: &impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render()
        .status(axum::http::StatusCode::NOT_FOUND)
        .view(v, "404.html", json!({"some": "value"}))
}
