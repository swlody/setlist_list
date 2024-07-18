#![allow(clippy::unused_async)]
use axum::{
    http::{Method, StatusCode},
    routing::any,
};
use loco_rs::prelude::*;

use crate::{initializers::minijinja_view_engine::MiniJinjaView, views};

pub async fn root(ViewEngine(v): ViewEngine<MiniJinjaView>) -> Result<impl IntoResponse> {
    let random = crate::models::index::random_string();
    views::index::root(&v, &random)
}

async fn not_found(method: Method, ViewEngine(v): ViewEngine<MiniJinjaView>) -> Result<Response> {
    if method == Method::GET {
        views::index::not_found(&v)
    } else {
        Ok((StatusCode::NOT_FOUND, "").into_response())
    }
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/", get(root))
        .add("/404", any(not_found))
}
