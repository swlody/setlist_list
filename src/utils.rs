use axum::http::uri::PathAndQuery;
use axum_htmx::HX_REDIRECT;
use cookie::Cookie;
use loco_rs::prelude::*;

use crate::models::users;

#[must_use]
pub fn get_username(jwt_user: Option<auth::JWTWithUser<users::Model>>) -> Option<String> {
    jwt_user.map(|jwt_user| jwt_user.user.username)
}

pub fn hx_redirect(redirect_to: &PathAndQuery) -> Result<Response> {
    format::RenderBuilder::new()
        .header(HX_REDIRECT, redirect_to.path())
        .empty()
}

pub fn redirect_with_cookies(
    redirect_to: &PathAndQuery,
    cookies: &[Cookie<'_>],
    hx: bool,
) -> Result<Response> {
    let builder = format::RenderBuilder::new().cookies(cookies)?;
    if hx {
        builder.header(HX_REDIRECT, redirect_to.path()).empty()
    } else {
        builder.redirect(redirect_to.path())
    }
}
