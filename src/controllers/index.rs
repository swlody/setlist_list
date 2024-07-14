#![allow(clippy::unused_async)]
use loco_rs::prelude::*;

pub async fn root(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    crate::views::index::root(&v)
}

async fn not_found(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    crate::views::index::not_found(&v)
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(root))
        .add("/404", get(not_found))
}
