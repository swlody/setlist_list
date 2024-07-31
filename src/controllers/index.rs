use loco_rs::prelude::*;

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView, models::users, utils::get_username, views,
};

pub async fn root(
    jwt_user: Option<auth::JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
) -> Result<impl IntoResponse> {
    let user_name = get_username(jwt_user).unwrap_or_default();
    views::index::root(&v, &user_name)
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(root))
}
