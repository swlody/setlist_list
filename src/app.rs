use std::{fs::File, path::Path};

use async_trait::async_trait;
use axum::{
    extract::OriginalUri,
    http::{Method, StatusCode},
    Router,
};
use loco_rs::{
    app::{AppContext, Hooks, Initializer},
    boot::{create_app, BootResult, StartMode},
    cache,
    controller::AppRoutes,
    environment::Environment,
    prelude::*,
    worker::Processor,
    Result,
};
use sqlx::PgPool;

use crate::{
    controllers,
    initializers::{self, minijinja_view_engine::MiniJinjaView},
    models::users,
    utils::get_username,
    views,
};

// TODO make this generic
pub async fn seed_users(db: &PgPool, users_path: &str) -> Result<()> {
    let users_loader: Vec<serde_json::Value> = serde_yaml::from_reader(File::open(users_path)?)?;

    for user in users_loader {
        let user: users::Model = serde_json::from_value(user)?;
        sqlx::query!(
            "INSERT into USERS (id, email, password, api_key, username, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            user.id,
            user.email,
            user.password,
            user.api_key,
            user.username,
            user.created_at,
            user.updated_at
        )
        .execute(db)
        .await?;
    }

    Ok(())
}

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
        create_app::<Self>(mode, environment).await
    }

    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![Box::new(
            initializers::minijinja_view_engine::ViewEngineInitializer,
        )])
    }

    fn routes(_ctx: &AppContext) -> AppRoutes {
        AppRoutes::with_default_routes()
            .add_route(controllers::sets::routes())
            .add_route(controllers::auth::routes())
            .add_route(controllers::index::routes())
            .add_route(controllers::user::routes())
    }

    async fn before_routes(_ctx: &AppContext) -> Result<Router<AppContext>> {
        async fn fallback_handler(
            jwt_user: Option<auth::JWTWithUser<users::Model>>,
            ViewEngine(v): ViewEngine<MiniJinjaView>,
            uri: OriginalUri,
            method: Method,
        ) -> impl IntoResponse {
            tracing::debug!("Returning 404 for {} on {}", method, uri.path());

            let user_name = get_username(jwt_user).unwrap_or_default();
            if method == Method::GET {
                views::index::not_found(&v, &user_name)
            } else {
                Ok((StatusCode::NOT_FOUND, "").into_response())
            }
        }

        let router = Router::new().fallback(fallback_handler);
        Ok(router)
    }

    fn connect_workers<'a>(_p: &'a mut Processor, _ctx: &'a AppContext) {
        // p.register(DownloadWorker::build(ctx));
    }

    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        Ok(AppContext {
            // TODO switch to redis?
            cache: cache::Cache::new(cache::drivers::inmem::new()).into(),
            ..ctx
        })
    }

    async fn migrate(db: &PgPool) -> Result<()> {
        sqlx::migrate!().run(db).await?;
        Ok(())
    }

    async fn truncate(db: &PgPool) -> Result<()> {
        sqlx::query!("TRUNCATE users").execute(db).await?;
        sqlx::query!("TRUNCATE sets").execute(db).await?;

        Ok(())
    }

    async fn seed(db: &PgPool, base: &Path) -> Result<()> {
        seed_users(db, &base.join("users.yaml").display().to_string()).await?;
        Ok(())
    }
}
