//! # Database Operations
//!
//! This module defines functions and operations related to the application's
//! database interactions.

use std::{path::Path, str::FromStr as _, time::Duration};

use lazy_static::lazy_static;
use regex::Regex;
use sqlx::{ConnectOptions, PgPool};
use tracing::info;

use super::Result as AppResult;
use crate::{app::Hooks, config, errors::Error};

lazy_static! {
    // Getting the table name from the environment configuration.
    // For example:
    // postgres://loco:loco@localhost:5432/loco_app
    // mysql://loco:loco@localhost:3306/loco_app
    // the results will be loco_app
    pub static ref EXTRACT_DB_NAME: Regex = Regex::new(r"/([^/]+)$").unwrap();
}

/// Verifies a user has access to data within its database
///
/// # Errors
///
/// This function will return an error if IO fails
#[allow(clippy::match_wildcard_for_single_variants)]
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

    if config.dangerously_truncate {
        info!("truncating tables");
        H::truncate(db).await?;
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

// / Seed the database with data from a specified file.
// / Seeds open the file path and insert all file content into the DB.
// /
// / The file content should be equal to the DB field parameters.
// /
// / # Errors
// /
// / Returns a [`AppResult`] if could not render the path content into
// / [`Vec<serde_json::Value>`] or could not inset the vector to DB.
// #[allow(clippy::type_repetition_in_bounds)]
// pub async fn seed<A>(db: &DatabaseConnection, path: &str) -> AppResult<()>
// where
//     <<A as ActiveModelTrait>::Entity as EntityTrait>::Model:
// IntoActiveModel<A>,     for<'de> <<A as ActiveModelTrait>::Entity as
// EntityTrait>::Model: serde::de::Deserialize<'de>,     A:
// sea_orm::ActiveModelTrait + Send + Sync,     sea_orm::Insert<A>: Send + Sync,
// // Add this Send bound {
//     let loader: Vec<serde_json::Value> =
// serde_yaml::from_reader(File::open(path)?)?;

//     let mut users: Vec<A> = vec![];
//     for user in loader {
//         users.push(A::from_json(user)?);
//     }

//     <A as ActiveModelTrait>::Entity::insert_many(users)
//         .exec(db)
//         .await?;

//     Ok(())
// }

/// Execute seed from the given path
///
/// # Errors
///
/// when seed process is fails
pub async fn run_app_seed<H: Hooks>(db: &PgPool, path: &Path) -> AppResult<()> {
    H::seed(db, path).await
}
