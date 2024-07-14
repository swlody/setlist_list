#![allow(clippy::unused_async)]
use axum::{
    http::{Method, StatusCode},
    routing::any,
};
use loco_rs::prelude::*;

pub async fn root(ViewEngine(v): ViewEngine<TeraView>) -> Result<impl IntoResponse> {
    crate::views::index::root(&v)
}

async fn not_found(method: Method, ViewEngine(v): ViewEngine<TeraView>) -> Result<Response> {
    if method == Method::GET {
        crate::views::index::not_found(&v)
    } else {
        Ok((StatusCode::NOT_FOUND, "").into_response())
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(root))
        .add("/404", any(not_found))
}
