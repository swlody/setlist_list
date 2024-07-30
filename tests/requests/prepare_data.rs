use axum::http::{header, HeaderName, HeaderValue};
use eyre::ContextCompat as _;
use loco_rs::{app::AppContext, TestServer};
use setlist_list::models::users;

const USER_PASSWORD: &str = "1234";

pub struct LoggedInUser {
    pub user: users::Model,
    pub _token: String,
}

pub async fn init_user_login(
    request: &TestServer,
    ctx: &AppContext,
    username: &str,
    email: &str,
) -> eyre::Result<LoggedInUser> {
    let register_payload = serde_json::json!({
        "username": username,
        "email": email,
        "password": USER_PASSWORD
    });

    //Creating a new user
    request.post("/register").json(&register_payload).await;
    let user = users::Model::find_by_email(&ctx.db, email).await?;

    request
        .get(&format!(
            "/verify_email?token={}",
            user.email_verification_token
                .context("unable to get email verification token")?
        ))
        .await;

    let response = request
        .post("/login")
        .json(&serde_json::json!({
            "email": email,
            "password": USER_PASSWORD
        }))
        .await;

    // TODO clean this up you maniac
    let token = response
        .headers()
        .get(header::SET_COOKIE)
        .context("unable to get COOKIE header")?
        .to_str()?
        .split_once("=")
        .context("cookie header does not have value")?
        .1
        .split_once(";")
        .context("cookie header does not end in semicolon")?
        .0
        .to_string();

    Ok(LoggedInUser {
        user: users::Model::find_by_email(&ctx.db, email).await?,
        _token: token,
    })
}

pub fn _auth_header(token: &str) -> eyre::Result<(HeaderName, HeaderValue)> {
    let auth_header_value = HeaderValue::from_str(&format!("Bearer {}", &token))?;

    Ok((HeaderName::from_static("authorization"), auth_header_value))
}
