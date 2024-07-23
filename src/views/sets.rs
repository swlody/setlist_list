use loco_rs::prelude::*;

use crate::models::_entities::sets;

/// Render a list view of sets.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn list(v: &impl ViewRenderer, items: &Vec<sets::Model>, user_name: &str) -> Result<Response> {
    format::render().view(
        v,
        "sets/list.html",
        serde_json::json!({"items": items, "username": user_name}),
    )
}

/// Render a single sets view.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn show(v: &impl ViewRenderer, item: &sets::Model, user_name: &str) -> Result<Response> {
    format::render().view(
        v,
        "sets/show.html",
        serde_json::json!({"item": item, "username": user_name}),
    )
}

/// Render a sets create form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn create(v: &impl ViewRenderer, user_name: &str) -> Result<Response> {
    format::render().view(
        v,
        "sets/create.html",
        serde_json::json!({"username": user_name}),
    )
}

/// Render a sets edit form.
///
/// # Errors
///
/// When there is an issue with rendering the view.
pub fn edit(v: &impl ViewRenderer, item: &sets::Model, user_name: &str) -> Result<Response> {
    format::render().view(
        v,
        "sets/edit.html",
        serde_json::json!({"item": item, "username": user_name}),
    )
}
