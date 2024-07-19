#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView, models::users, utils::get_user_name, views,
};

pub async fn login(
    jwt_user: Option<auth::JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
) -> Result<impl IntoResponse> {
    let user_name = get_user_name(jwt_user);
    views::auth::login(&v, &user_name)
}

pub fn routes() -> Routes {
    Routes::new().add("/login", get(login))
}
