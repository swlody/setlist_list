use loco_rs::prelude::*;
use serde_json::json;

pub fn root(v: &impl ViewRenderer) -> Result<impl IntoResponse> {
    format::render().view(v, "index.html", json!({"some": "value"}))
}
