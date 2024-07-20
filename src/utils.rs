use crate::models::users;
use ::cookie::Cookie;
use loco_rs::prelude::*;

#[must_use]
pub fn get_user_name(jwt_user: Option<auth::JWTWithUser<users::Model>>) -> Option<String> {
    jwt_user.map(|jwt_user| jwt_user.user.name)
}

pub fn hx_redirect(redirect_to: &str) -> Result<Response> {
    format::RenderBuilder::new()
        .header("HX-Redirect", redirect_to)
        .empty()
}

pub fn hx_redirect_with_cookies(redirect_to: &str, cookies: &[Cookie<'_>]) -> Result<Response> {
    format::RenderBuilder::new()
        .header("HX-Redirect", redirect_to)
        .cookies(cookies)?
        .empty()
}
