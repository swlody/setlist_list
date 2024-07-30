use axum::http::header;
use insta::{assert_debug_snapshot, with_settings};
use loco_rs::testing;
use setlist_list::{app::App, models::users};
use sqlx::PgPool;

use crate::utils::get_random_user_email;

use super::prepare_data;

// TODO: see how to dedup / extract this to app-local test utils
// not to framework, because that would require a runtime dep on insta
macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("auth_request");
        let _guard = settings.bind_to_scope();
    };
}

#[sqlx::test]
async fn can_register(pool: PgPool) {
    configure_insta!();

    testing::request::<App, _, _>(pool, |request, ctx| async move {
        let (username, email) = get_random_user_email();
        let payload = serde_json::json!({
            "username": username,
            "email": email,
            "password": "12341234"
        });

        let _response = request.post("/register").json(&payload).await;
        let saved_user = users::Model::find_by_email(&ctx.db, &email).await;

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!(saved_user);
        });

        with_settings!({
            filters => testing::cleanup_email()
        }, {
            assert_debug_snapshot!(ctx.mailer.unwrap().deliveries());
        });
    })
    .await;
}

async fn login_with_verify(pool: PgPool, test_name: &str, password: &str) {
    configure_insta!();

    testing::request::<App, _, _>(pool, |request, ctx| async move {
        let (username, email) = get_random_user_email();
        let register_payload = serde_json::json!({
            "username": username,
            "email": email,
            "password": "12341234"
        });

        //Creating a new user
        _ = request.post("/register").json(&register_payload).await;

        let user = users::Model::find_by_email(&ctx.db, &email).await.unwrap();
        request
            .get(&format!(
                "/verify_email?token={}",
                user.email_verification_token.unwrap()
            ))
            .await;

        // Make sure email_verified_at is set
        assert!(users::Model::find_by_email(&ctx.db, &email)
            .await
            .unwrap()
            .email_verified_at
            .is_some());

        //verify user request
        let response = request
            .post("/login")
            .json(&serde_json::json!({
                "email": email,
                "password": password
            }))
            .await;

        let has_cookie = response.headers().get(header::SET_COOKIE).is_some();

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!(test_name, (response.status_code(), has_cookie));
        });
    })
    .await;
}

#[sqlx::test]
async fn can_login_with_verify_valid_password(pool: PgPool) {
    login_with_verify(pool, "login_with_valid_password", "12341234").await
}

#[sqlx::test]
async fn can_login_with_verify_invalid_password(pool: PgPool) {
    login_with_verify(pool, "login_with_invalid_password", "invalid-password").await
}

#[sqlx::test]
async fn can_login_without_verify(pool: PgPool) {
    configure_insta!();

    testing::request::<App, _, _>(pool, |request, _ctx| async move {
        let (username, email) = get_random_user_email();
        let password = "12341234";
        let register_payload = serde_json::json!({
            "username": username,
            "email": email,
            "password": password
        });

        //Creating a new user
        _ = request.post("/register").json(&register_payload).await;

        //verify user request
        let response = request
            .post("/login")
            .json(&serde_json::json!({
                "email": email,
                "password": password
            }))
            .await;

        let has_cookie = response.headers().get(header::SET_COOKIE).is_some();

        with_settings!({
            filters => testing::cleanup_user_model()
        }, {
            assert_debug_snapshot!((response.status_code(), has_cookie));
        });
    })
    .await;
}

#[sqlx::test]
async fn can_reset_password(pool: PgPool) {
    configure_insta!();

    testing::request::<App, _, _>(pool, |request, ctx| async move {
        let (username, email) = get_random_user_email();
        let login_data = prepare_data::init_user_login(&request, &ctx, &username, &email).await;

        let forgot_payload = serde_json::json!({
            "email": login_data.user.email,
        });
        _ = request.post("/forgot_password").json(&forgot_payload).await;

        let user = users::Model::find_by_email(&ctx.db, &login_data.user.email)
            .await
            .unwrap();
        assert!(user.reset_token.is_some());
        assert!(user.reset_sent_at.is_some());

        let new_password = "new-password";
        let reset_payload = serde_json::json!({
            "token": user.reset_token,
            "password": new_password,
        });

        let reset_response = request.post("/reset_password").json(&reset_payload).await;

        let user = users::Model::find_by_email(&ctx.db, &user.email)
            .await
            .unwrap();

        assert!(user.reset_token.is_none());
        assert!(user.reset_sent_at.is_none());

        assert_debug_snapshot!((reset_response.status_code(), reset_response.text()));

        let response = request
            .post("/login")
            .json(&serde_json::json!({
                "email": user.email,
                "password": new_password
            }))
            .await;

        assert_eq!(response.status_code(), 303);

        with_settings!({
            filters => testing::cleanup_email()
        }, {
            assert_debug_snapshot!(ctx.mailer.unwrap().deliveries());
        });
    })
    .await;
}
