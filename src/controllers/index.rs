#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

use crate::{initializers::minijinja_view_engine::MiniJinjaView, views};

pub async fn root(ViewEngine(v): ViewEngine<MiniJinjaView>) -> Result<impl IntoResponse> {
    let random = crate::models::index::random_string();
    views::index::root(&v, &random)
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(root))
}
