use crate::models::users;
use ::cookie::Cookie;
use axum::http::uri::PathAndQuery;
use axum_htmx::HX_REDIRECT;
use loco_rs::prelude::*;

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
) -> Result<Response> {
    format::RenderBuilder::new()
        .header(HX_REDIRECT, redirect_to.path())
        .cookies(cookies)?
        .redirect(redirect_to.path())
}
