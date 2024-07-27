use loco_rs::prelude::*;

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
