//! This module contains the core components and traits for building a web
//! server application.
use std::sync::Arc;

use async_trait::async_trait;
use axum::Router as AxumRouter;
use sqlx::PgPool;

#[cfg(feature = "channels")]
use crate::controller::channels::AppChannels;
use crate::{
    boot::{BootResult, ServeParams, StartMode},
    cache::{self},
    config::{self, Config},
    controller::AppRoutes,
    environment::Environment,
    mailer::EmailSender,
    storage::Storage,
    worker::{Pool, Processor, RedisConnectionManager},
    Result,
};

/// Represents the application context for a web server.
///
/// This struct encapsulates various components and configurations required by
/// the web server to operate. It is typically used to store and manage shared
/// resources and settings that are accessible throughout the application's
/// lifetime.
#[derive(Clone)]
#[allow(clippy::module_name_repetitions)]
pub struct AppContext {
    /// The environment in which the application is running.
    pub environment: Environment,

    /// A database connection used by the application.
    pub db: sqlx::PgPool,
    /// An optional connection pool for Queue, for worker tasks
    pub queue: Option<Pool<RedisConnectionManager>>,
    /// Configuration settings for the application
    pub config: Config,
    /// An optional email sender component that can be used to send email.
    pub mailer: Option<EmailSender>,
    // An optional storage instance for the application
    pub storage: Arc<Storage>,
    // Cache instance for the application
    pub cache: Arc<cache::Cache>,
}

/// A trait that defines hooks for customizing and extending the behavior of a
/// web server application.
///
/// Users of the web server application should implement this trait to customize
/// the application's routing, worker connections, task registration, and
/// database actions according to their specific requirements and use cases.
#[async_trait]
pub trait Hooks {
    /// Defines the composite app version
    #[must_use]
    fn app_version() -> String {
        "dev".to_string()
    }
    /// Defines the crate name
    ///
    /// Example
    /// ```rust
    /// fn app_name() -> &'static str {
    ///     env!("CARGO_CRATE_NAME")
    /// }
    /// ```
    fn app_name() -> &'static str;

    /// Initializes and boots the application based on the specified mode and
    /// environment.
    ///
    /// The boot initialization process may vary depending on whether a DB
    /// migrator is used or not.
    ///
    /// # Examples
    ///
    /// With DB:
    /// ```rust,ignore
    /// async fn boot(mode: StartMode, environment: &str) -> Result<BootResult> {
    ///     create_app::<Self, Migrator>(mode, environment).await
    /// }
    /// ````
    ///
    /// Without DB:
    /// ```rust,ignore
    /// async fn boot(mode: StartMode, environment: &str) -> Result<BootResult> {
    ///     create_app::<Self>(mode, environment).await
    /// }
    /// ````
    ///
    ///
    /// # Errors
    /// Could not boot the application
    async fn boot(
        mode: StartMode,
        environment: &Environment,
        pool: Option<PgPool>,
    ) -> Result<BootResult>;

    /// Start serving the Axum web application on the specified address and
    /// port.
    ///
    /// # Returns
    /// A Result indicating success () or an error if the server fails to start.
    async fn serve(app: AxumRouter, server_config: ServeParams) -> Result<()> {
        tracing::info!(
            "Server started on {}:{}",
            server_config.binding,
            server_config.port
        );
        let listener = tokio::net::TcpListener::bind(&format!(
            "{}:{}",
            server_config.binding, server_config.port
        ))
        .await?;

        axum::serve(listener, app).await?;

        Ok(())
    }

    /// Override and return `Ok(true)` to provide an alternative logging and
    /// tracing stack of your own.
    /// When returning `Ok(true)`, Loco will *not* initialize its own logger,
    /// so you should set up a complete tracing and logging stack.
    ///
    /// # Errors
    /// If fails returns an error
    fn init_logger(_config: &config::Config, _env: &Environment) -> Result<bool> {
        Ok(false)
    }

    /// Returns the initial Axum router for the application, allowing the user
    /// to control the construction of the Axum router. This is where a fallback
    /// handler can be installed before middleware or other routes are added.
    ///
    /// # Errors
    /// Return an [`Result`] when the router could not be created
    async fn before_routes(_ctx: &AppContext) -> Result<AxumRouter<AppContext>> {
        Ok(AxumRouter::new())
    }

    /// Invoke this function after the Loco routers have been constructed. This
    /// function enables you to configure custom Axum logics, such as layers,
    /// that are compatible with Axum.
    ///
    /// # Errors
    /// Axum router error
    async fn after_routes(router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        Ok(router)
    }

    /// Provide a list of initializers
    /// An initializer can be used to seamlessly add functionality to your app
    /// or to initialize some aspects of it.
    async fn initializers(_ctx: &AppContext) -> Result<Vec<Box<dyn Initializer>>> {
        Ok(vec![])
    }

    /// Calling the function before run the app
    /// You can now code some custom loading of resources or other things before
    /// the app runs
    async fn before_run(_app_context: &AppContext) -> Result<()> {
        Ok(())
    }

    /// Defines the application's routing configuration.
    fn routes(_ctx: &AppContext) -> AppRoutes;

    // Provides the options to change Loco [`AppContext`] after initialization.
    async fn after_context(ctx: AppContext) -> Result<AppContext> {
        Ok(ctx)
    }

    #[cfg(feature = "channels")]
    /// Register channels endpoints to the application routers
    fn register_channels(_ctx: &AppContext) -> AppChannels;

    /// Connects custom workers to the application using the provided
    /// [`Processor`] and [`AppContext`].
    fn connect_workers<'a>(p: &'a mut Processor, ctx: &'a AppContext);

    // Runs migrations on the database.
    async fn migrate(db: &PgPool) -> Result<()>;

    // Cleanup any outstanding handles - i.e. database connections.
    async fn cleanup(ctx: &AppContext) -> Result<()>;
}

/// An initializer.
/// Initializers should be kept in `src/initializers/`
#[async_trait]
// <snip id="initializers-trait">
pub trait Initializer: Sync + Send {
    /// The initializer name or identifier
    fn name(&self) -> String;

    /// Occurs after the app's `before_run`.
    /// Use this to for one-time initializations, load caches, perform web
    /// hooks, etc.
    async fn before_run(&self, _app_context: &AppContext) -> Result<()> {
        Ok(())
    }

    /// Occurs after the app's `after_routes`.
    /// Use this to compose additional functionality and wire it into an Axum
    /// Router
    async fn after_routes(&self, router: AxumRouter, _ctx: &AppContext) -> Result<AxumRouter> {
        Ok(router)
    }
}
// </snip>
