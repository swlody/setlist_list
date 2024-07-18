use std::path::Path;

use async_trait::async_trait;
use axum::{
    extract::OriginalUri,
    http::{Method, StatusCode},
    Router,
};
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    boot::{create_app, BootResult, StartMode},
    controller::AppRoutes,
    db::{self, truncate_table},
    environment::Environment,
    prelude::*,
    task::Tasks,
    worker::{AppWorker, Processor},
    Result,
};
use migration::Migrator;
use sea_orm::DatabaseConnection;

use crate::{
    controllers,
    initializers::{self, minijinja_view_engine::MiniJinjaView},
    models::_entities::users,
    tasks, views,
    workers::downloader::DownloadWorker,
};

pub struct App;
#[async_trait]
impl Hooks for App {
    fn app_name() -> &'static str {
        env!("CARGO_CRATE_NAME")
    }

    fn app_version() -> String {
        format!(
            "{} ({})",
            env!("CARGO_PKG_VERSION"),
            option_env!("BUILD_SHA")
                .or(option_env!("GITHUB_SHA"))
                .unwrap_or("dev")
        )
    }

    async fn boot(mode: StartMode, environment: &Environment) -> Result<BootResult> {
        create_app::<Self, Migrator>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::minijinja_view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::auth::routes())
            .add_route(controllers::index::routes())
            .add_route(controllers::auth_api::routes())
            .add_route(controllers::user_api::routes())
    }

    async fn after_routes(router: Router, _ctx: &AppContext) -> Result<Router> {
        async fn fallback_handler(
            ViewEngine(v): ViewEngine<MiniJinjaView>,
            uri: OriginalUri,
            method: Method,
        ) -> impl IntoResponse {
            tracing::debug!("Returning 404 for {} on {}", method, uri.path());
            if method == Method::GET {
                views::index::not_found(&v)
            } else {
                Ok((StatusCode::NOT_FOUND, "").into_response())
            }
        }

        tracing::info!("Adding 404 fallback route");
        let router = router.fallback(fallback_handler);
        Ok(router)
    }

    fn connect_workers<'a>(p: &'a mut Processor, ctx: &'a AppContext) {
        p.register(DownloadWorker::build(ctx));
    }

    fn register_tasks(tasks: &mut Tasks) {
        tasks.register(tasks::seed::SeedData);
    }

    async fn truncate(db: &DatabaseConnection) -> Result<()> {
        truncate_table(db, users::Entity).await?;
        Ok(())
    }

    async fn seed(db: &DatabaseConnection, base: &Path) -> Result<()> {
        db::seed::<users::ActiveModel>(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }
}
