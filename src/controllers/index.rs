#![allow(clippy::unused_async)]
use axum::debug_handler;
use loco_rs::prelude::*;

#[debug_handler]
pub async fn root(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    crate::views::index::root(&v)
}

pub fn routes() -> Routes {
    Routes::new().add("/", get(root))
}
