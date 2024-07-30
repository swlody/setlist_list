use insta::assert_debug_snapshot;
use loco_rs::{model::ModelError, testing};
use setlist_list::{
    app::App,
    models::users::{Model, RegisterParams},
};
use sqlx::PgPool;
use uuid::uuid;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        settings.set_snapshot_suffix("users");
        let _guard = settings.bind_to_scope();
    };
}

#[sqlx::test]
async fn can_create_with_password(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let params = RegisterParams {
        email: "test@framework.com".to_string(),
        password: "1234".to_string(),
        username: "framework".to_string(),
    };
    let res = Model::create_with_password(&boot.app_context.db, &params).await;

    insta::with_settings!({
        filters => testing::cleanup_user_model()
    }, {
        assert_debug_snapshot!(res);
    });
}

#[sqlx::test(fixtures("users"))]
async fn handle_create_with_password_with_duplicate(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let new_user: Result<Model, ModelError> = Model::create_with_password(
        &boot.app_context.db,
        &RegisterParams {
            email: "user1@example.com".to_string(),
            password: "1234".to_string(),
            username: "framework".to_string(),
        },
    )
    .await;
    assert!(new_user.is_err());
}

#[sqlx::test(fixtures("users"))]
async fn can_find_by_email(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let existing_user = Model::find_by_email(&boot.app_context.db, "user1@example.com").await;
    let non_existing_user_results =
        Model::find_by_email(&boot.app_context.db, "un@existing-email.com").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);
}

#[sqlx::test(fixtures("users"))]
async fn can_find_by_id(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let existing_user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await;
    let non_existing_user_results =
        Model::find_by_email(&boot.app_context.db, "23232323-2323-2323-2323-232323232323").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);
}

#[sqlx::test(fixtures("users"))]
async fn can_verification_token(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let mut user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await
    .unwrap();

    assert!(user.email_verification_sent_at.is_none());
    assert!(user.email_verification_token.is_none());

    assert!(user
        .set_email_verification_sent(&boot.app_context.db)
        .await
        .is_ok());

    let user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await
    .unwrap();

    assert!(user.email_verification_sent_at.is_some());
    assert!(user.email_verification_token.is_some());
}

#[sqlx::test(fixtures("users"))]
async fn can_set_forgot_password_sent(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let mut user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await
    .unwrap();

    assert!(user.reset_sent_at.is_none());
    assert!(user.reset_token.is_none());

    assert!(user
        .set_forgot_password_sent(&boot.app_context.db)
        .await
        .is_ok());

    let user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await
    .unwrap();

    assert!(user.reset_sent_at.is_some());
    assert!(user.reset_token.is_some());
}

#[sqlx::test(fixtures("users"))]
async fn can_verified(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let mut user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await
    .unwrap();

    assert!(user.email_verified_at.is_none());

    assert!(user.verified(&boot.app_context.db).await.is_ok());

    let user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await
    .unwrap();

    assert!(user.email_verified_at.is_some());
}

#[sqlx::test(fixtures("users"))]
async fn can_reset_password(pool: PgPool) {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await.unwrap();

    let user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await
    .unwrap();

    assert!(user.verify_password("12341234"));

    assert!(user
        .clone()
        .reset_password(&boot.app_context.db, "new-password")
        .await
        .is_ok());

    assert!(Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111")
    )
    .await
    .unwrap()
    .verify_password("new-password"));
}
