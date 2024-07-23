#![allow(clippy::unused_async)]
use std::time::SystemTime;

use ::cookie::CookieBuilder;
use axum::{debug_handler, http::uri::PathAndQuery};
use loco_rs::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    initializers::minijinja_view_engine::MiniJinjaView,
    mailers::auth::AuthMailer,
    models::users,
    models::users::{LoginParams, RegisterParams},
    utils::get_user_name,
    utils::{hx_redirect, hx_redirect_with_cookies},
    views,
};

#[derive(Debug, Deserialize, Serialize)]
pub struct VerifyParams {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ForgotParams {
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResetParams {
    pub token: String,
    pub password: String,
}

/// Register function creates a new user with the given parameters and sends a
/// welcome email to the user
#[debug_handler]
async fn register(
    State(ctx): State<AppContext>,
    Json(params): Json<RegisterParams>,
) -> Result<Response> {
    let res = users::Model::create_with_password(&ctx.db, &params).await;

    let user = match res {
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

    let user = user
        .into_active_model()
        .set_email_verification_sent(&ctx.db)
        .await?;

    AuthMailer::send_welcome(&ctx, &user).await?;

    hx_redirect(&PathAndQuery::from_static("/"))
}

/// Verify register user. if the user not verified his email, he can't login to
/// the system.
#[debug_handler]
async fn verify(
    State(ctx): State<AppContext>,
    Json(params): Json<VerifyParams>,
) -> Result<Response> {
    let user = users::Model::find_by_verification_token(&ctx.db, &params.token).await?;

    if user.email_verified_at.is_some() {
        tracing::info!(pid = user.pid.to_string(), "user already verified");
    } else {
        let active_model = user.into_active_model();
        let user = active_model.verified(&ctx.db).await?;
        tracing::info!(pid = user.pid.to_string(), "user verified");
    }

    format::json(())
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
    let Ok(user) = users::Model::find_by_email(&ctx.db, &params.email).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        return hx_redirect(&PathAndQuery::from_static("/login"));
    };

    let user = user
        .into_active_model()
        .set_forgot_password_sent(&ctx.db)
        .await?;

    AuthMailer::forgot_password(&ctx, &user).await?;

    hx_redirect(&PathAndQuery::from_static("/login"))
}

/// reset user password by the given parameters
#[debug_handler]
async fn reset(State(ctx): State<AppContext>, Json(params): Json<ResetParams>) -> Result<Response> {
    let Ok(user) = users::Model::find_by_reset_token(&ctx.db, &params.token).await else {
        // we don't want to expose our users email. if the email is invalid we still
        // returning success to the caller
        tracing::info!("reset token not found");

        return format::json(());
    };
    user.into_active_model()
        .reset_password(&ctx.db, &params.password)
        .await?;

    format::json(())
}

/// Creates a user login and returns a token
#[debug_handler]
async fn login(State(ctx): State<AppContext>, Json(params): Json<LoginParams>) -> Result<Response> {
    let user = users::Model::find_by_email(&ctx.db, &params.email).await?;

    let valid = user.verify_password(&params.password);

    if !valid {
        return unauthorized("unauthorized!");
    }

    let jwt_secret = ctx.config.get_jwt_config()?;

    let token = user
        .generate_jwt(&jwt_secret.secret, &jwt_secret.expiration)
        .or_else(|_| unauthorized("unauthorized!"))?;

    // TODO remove unwraps (should never fail but still)
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let jwt_expiration = i64::try_from(now + jwt_secret.expiration).unwrap();
    let expiry_time = time::OffsetDateTime::from_unix_timestamp(jwt_expiration).unwrap();

    let cookie = CookieBuilder::new("token", token)
        .path("/")
        .expires(expiry_time)
        .http_only(true)
        .secure(true)
        .same_site(cookie::SameSite::Lax)
        .build();

    hx_redirect_with_cookies(&PathAndQuery::from_static("/"), &[cookie])
}

#[debug_handler]
async fn logout(_auth: auth::JWT, State(_ctx): State<AppContext>) -> Result<Response> {
    let now = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    let jwt_expiration = i64::try_from(now + 10).unwrap();
    let expiry_time = time::OffsetDateTime::from_unix_timestamp(jwt_expiration).unwrap();

    let cookie = CookieBuilder::new("token", "deleted")
        .path("/")
        .expires(expiry_time)
        .same_site(cookie::SameSite::None)
        .build();

    hx_redirect_with_cookies(&PathAndQuery::from_static("/"), &[cookie])
}

pub async fn login_page(
    jwt_user: Option<auth::JWTWithUser<users::Model>>,
    ViewEngine(v): ViewEngine<MiniJinjaView>,
) -> Result<impl IntoResponse> {
    let user_name = get_user_name(jwt_user).unwrap_or_default();
    views::auth::login(&v, &user_name)
}

pub fn routes() -> Routes {
    Routes::new()
        .add("/login", get(login_page).post(login))
        .add("/register", post(register))
        .add("/verify", post(verify))
        .add("/forgot", post(forgot))
        .add("/reset", post(reset))
        .add("/logout", post(logout))
}
