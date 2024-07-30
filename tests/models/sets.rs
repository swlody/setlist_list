use insta::assert_debug_snapshot;
use loco_rs::testing;
use setlist_list::app::App;
use setlist_list::models::sets::Model;
use sqlx::PgPool;
use uuid::uuid;

macro_rules! configure_insta {
    ($($expr:expr),*) => {
        let mut settings = insta::Settings::clone_current();
        settings.set_prepend_module_to_snapshot(false);
        let _guard = settings.bind_to_scope();
    };
}

#[sqlx::test(fixtures("sets"))]
async fn can_find_by_id(pool: PgPool) -> eyre::Result<()> {
    configure_insta!();

    let boot = testing::boot_test::<App>(pool).await?;

    let item = Model::find_by_id(
        &boot.app_context.db,
        uuid!("33333333-3333-3333-3333-333333333333"),
    )
    .await?;

    // snapshot the result:
    assert_debug_snapshot!(item);
    Ok(())
}
