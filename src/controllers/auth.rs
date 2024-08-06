use std::time::SystemTime;

use ::cookie::CookieBuilder;
use axum::{debug_handler, extract::Query, http::uri::PathAndQuery};
use loco_rs::prelude::*;
use secrecy::Secret;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView,
    mailers::auth::AuthMailer,
    models::users::{self, LoginParams, RegisterParams},
    utils::{get_username, hx_redirect, redirect_with_cookies},
    views,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: Uuid,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: Uuid,
    #[serde(serialize_with = "loco_rs::utils::stars")]
    pub password: Secret<String>,
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let mut user = match res {
        Ok(user) => user,
        Err(err) => {
            tracing::info!(
                message = err.to_string(),
                user_email = &params.email,
                "could not register user",
            );
            return format::json(());
        }
    };

    user.set_email_verification_sent(&ctx.db).await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    hx_redirect(&PathAndQuery::from_static("/register/success"))
}

fn login_cookie_redirect(ctx: &AppContext, user: &users::Model, hx: bool) -> Result<Response> {
    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| Error::Any(e.into()))?
        .as_secs();
    let jwt_expiration =
        i64::try_from(now + jwt_secret.expiration).map_err(|e| Error::Any(e.into()))?;
    let expiry_time = time::OffsetDateTime::from_unix_timestamp(jwt_expiration)
        .map_err(|e| Error::Any(e.into()))?;

    let cookie = CookieBuilder::new("token", token)
        .path("/")
        .expires(expiry_time)
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Lax)
        .build();

    redirect_with_cookies(&PathAndQuery::from_static("/"), &[cookie], hx)
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(
    ViewEngine(v): ViewEngine<MiniJinjaView>,
    State(ctx): State<AppContext>,
    Query(VerifyParams { token }): Query<VerifyParams>,
) -> Result<Response> {
    let mut user = users::Model::find_by_verification_token(&ctx.db, token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(id = user.id.to_string(), "user already verified");
        views::index::unauthorized(&v)
    } else {
        user.verified(&ctx.db).await?;
        tracing::info!(id = user.id.to_string(), "user verified");
        // Since we are coming from a third part, (email), don't set hx-redirect
        login_cookie_redirect(&ctx, &user, false)
    }
}

/// In case the user forgot his password  this endpoints generate a forgot token
/// and send email to the user. In case the email not found in our DB, we are
/// returning a valid request for for security reasons (not exposing users DB
/// list).
#[debug_handler]
async fn forgot(
    State(ctx): State<AppContext>,
    Json(params): Json<ForgotParams>,
) -> Result<Response> {
    let Ok(mut user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return hx_redirect(&PathAndQuery::from_static("/login"));
    };

    user.set_forgot_password_sent(&ctx.db).await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    hx_redirect(&PathAndQuery::from_static("/login"))
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(mut user) = users::Model::find_by_reset_token(&ctx.db, params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.reset_password(&ctx.db, &params.password).await?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;

    if !user.verify_password(&params.password) {
        return unauthorized("unauthorized!");
    }

    if user.email_verified_at.is_none() {
        return unauthorized("unauthorized! email not verified");
    }

    login_cookie_redirect(&ctx, &user, true)
}

#[debug_handler]
async fn logout(_auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| Error::Any(e.into()))?
        .as_secs();
    let jwt_expiration = i64::try_from(now + 10).map_err(|e| Error::Any(e.into()))?;
    let expiry_time = time::OffsetDateTime::from_unix_timestamp(jwt_expiration)
        .map_err(|e| Error::Any(e.into()))?;

    let cookie = CookieBuilder::new("token", "deleted")
        .path("/")
        .expires(expiry_time)
        .same_site(cookie::SameSite::None)
        .build();

    redirect_with_cookies(&PathAndQuery::from_static("/"), &[cookie], true)
}

pub async fn login_page(
    jwt_user: Option<auth::JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
) -> Result<impl IntoResponse> {
    let user_name = get_username(jwt_user).unwrap_or_default();
    views::auth::login(&v, &user_name)
}

pub async fn registration_success_page(
    jwt_user: Option<auth::JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
) -> Result<impl IntoResponse> {
    let user_name = get_username(jwt_user).unwrap_or_default();
    views::auth::registration_success(&v, &user_name)
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/login", get(login_page).post(login))
        .add("/register", post(register))
        .add("/register/success", get(registration_success_page))
        .add("/verify_email", get(verify))
        .add("/forgot_password", post(forgot))
        .add("/reset_password", post(reset))
        .add("/logout", post(logout))
}
