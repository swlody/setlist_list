//! # Database Operations
//!
//! This module defines functions and operations related to the application's
//! database interactions.

use std::{str::FromStr as _, time::Duration};

use sqlx::{ConnectOptions, PgPool};
use tracing::info;

use super::Result as AppResult;
use crate::{app::Hooks, config, errors::Error};

/// Verifies a user has access to data within its database
///
/// # Errors
///
/// This function will return an error if IO fails
pub async fn verify_access(db: &PgPool) -> AppResult<()> {
    let res = sqlx::query("SELECT * FROM pg_catalog.pg_tables WHERE tableowner = current_user")
        .fetch_all(db)
        .await?;
    if res.is_empty() {
        return Err(Error::string(
            "current user has no access to tables in the database",
        ));
    }
    Ok(())
}
/// converge database logic
///
/// # Errors
///
///  an `AppResult`, which is an alias for `Result<(), AppError>`. It may
/// return an `AppError` variant representing different database operation
/// failures.
pub async fn converge<H: Hooks>(db: &PgPool, config: &config::Database) -> AppResult<()> {
    if config.auto_migrate {
        info!("auto migrating");
        H::migrate(db).await?;
    }
    Ok(())
}

/// Establish a connection to the database using the provided configuration
/// settings.
///
/// # Errors
///
/// Returns a [`sqlx::Error`] if an error occurs during the database
/// connection establishment.
pub async fn connect(config: &config::Database) -> Result<PgPool, sqlx::Error> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_millis(config.connect_timeout))
        .idle_timeout(Duration::from_millis(config.idle_timeout));

    let mut connect_options = sqlx::postgres::PgConnectOptions::from_str(&config.uri)?;
    if !config.enable_logging {
        connect_options = connect_options.disable_statement_logging();
    }

    pool.connect_with(connect_options).await
}
