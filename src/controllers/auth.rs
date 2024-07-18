#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{initializers::minijinja_view_engine::MiniJinjaView, views};

pub async fn login(ViewEngine(v): ViewEngine<MiniJinjaView>) -> Result<impl IntoResponse> {
    views::auth::login(&v)
}

pub fn routes() -> Routes {
    Routes::new().add("/login", get(login))
}
