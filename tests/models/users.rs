use insta::assert_debug_snapshot;
use loco_rs::{model::ModelError, testing};
use secrecy::Secret;
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
async fn can_create_with_password(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let params = RegisterParams {
        email: "test@framework.com".to_string(),
        password: Secret::new("1234".to_owned()),
        username: "framework".to_string(),
    };
    let res = Model::create_with_password(&boot.app_context.db, &params).await;

    insta::with_settings!({
        filters => testing::cleanup_user_model()
    }, {
        assert_debug_snapshot!(res);
    });

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn handle_create_with_password_with_duplicate(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let new_user: Result<Model, ModelError> = Model::create_with_password(
        &boot.app_context.db,
        &RegisterParams {
            email: "user1@example.com".to_string(),
            password: Secret::new("1234".to_owned()),
            username: "framework".to_string(),
        },
    )
    .await;
    assert!(new_user.is_err());

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn can_find_by_email(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let existing_user = Model::find_by_email(&boot.app_context.db, "user1@example.com").await;
    let non_existing_user_results =
        Model::find_by_email(&boot.app_context.db, "un@existing-email.com").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn can_find_by_id(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let existing_user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await;
    let non_existing_user_results =
        Model::find_by_email(&boot.app_context.db, "23232323-2323-2323-2323-232323232323").await;

    assert_debug_snapshot!(existing_user);
    assert_debug_snapshot!(non_existing_user_results);

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn can_verification_token(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let mut user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await?;

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
    .await?;

    assert!(user.email_verification_sent_at.is_some());
    assert!(user.email_verification_token.is_some());

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn can_set_forgot_password_sent(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let mut user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await?;

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
    .await?;

    assert!(user.reset_sent_at.is_some());
    assert!(user.reset_token.is_some());

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn can_verified(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let mut user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await?;

    assert!(user.email_verified_at.is_none());

    assert!(user.verified(&boot.app_context.db).await.is_ok());

    let user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await?;

    assert!(user.email_verified_at.is_some());

    Ok(())
}

#[sqlx::test(fixtures("users"))]
async fn can_reset_password(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let user = Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111"),
    )
    .await?;

    assert!(user.verify_password(&Secret::new("12341234".to_owned())));

    assert!(user
        .clone()
        .reset_password(
            &boot.app_context.db,
            &Secret::new("new-password".to_owned())
        )
        .await
        .is_ok());

    assert!(Model::find_by_id(
        &boot.app_context.db,
        uuid!("11111111-1111-1111-1111-111111111111")
    )
    .await?
    .verify_password(&Secret::new("new-password".to_owned())));

    Ok(())
}
