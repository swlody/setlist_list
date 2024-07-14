use std::path::Path;

use async_trait::async_trait;
use axum::extract::Request;
use axum::{
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
    controllers, initializers, models::_entities::users, tasks, workers::downloader::DownloadWorker,
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
            initializers::view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::index::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::user::routes())
    }

    async fn after_routes(router: Router, _ctx: &AppContext) -> Result<Router> {
        async fn fallback_handler(request: Request) -> Result<Response> {
            let method = request.method();
            if Method::GET == method {
                tracing::debug!("redirecting to 404 from: {}", request.uri());
                format::redirect("/404")
            } else {
                // ref: https://github.com/loco-rs/loco/blob/1f2401951f445ef1d71ce41f562ab3d0fb89bcd3/src/controller/app_routes.rs#L375-L399
                let request_id = uuid::Uuid::new_v4();
                let user_agent = request
                    .headers()
                    .get(axum::http::header::USER_AGENT)
                    .map_or("", |h| h.to_str().unwrap_or(""));

                tracing::error_span!(
                    "http-request",
                    "http.method" = tracing::field::display(request.method()),
                    "http.uri" = tracing::field::display(request.uri()),
                    "http.version" = tracing::field::debug(request.version()),
                    "http.user_agent" = tracing::field::display(user_agent),
                    request_id = tracing::field::display(request_id),
                )
                .in_scope(|| {
                    tracing::event!(
                        tracing::Level::DEBUG,
                        "returning 404 not found for unknown route"
                    );
                });

                Ok((StatusCode::NOT_FOUND, "").into_response())
            }
        }

        Ok(router.fallback(fallback_handler))
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
