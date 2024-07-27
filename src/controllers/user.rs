use auth::JWTWithUser;
use axum::debug_handler;
use loco_rs::prelude::*;

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView, models::users, utils::get_username, views,
};

#[debug_handler]
async fn user(
    jwt_user: Option<JWTWithUser<users::Model>>,
    Path(username): Path<String>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
) -> Result<Response> {
    let user = users::Model::find_by_username(&ctx.db, &username).await;
    let own_user = get_username(jwt_user).unwrap_or_default();
    if let Ok(user) = user {
        let sets = crate::models::sets::Model::list_by_creator_id(&ctx.db, user.id).await?;
        views::user::sets(&v, &user.username, &sets, &own_user)
    } else {
        views::index::not_found(&v, &own_user)
    }
}

pub fn routes() -> Routes {
    Routes::new().add("/user/:username", get(user))
}
