//! # Model Error Handling
//!
//! Useful when using `sea_orm` and want to propagate errors

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::validation::ModelValidationErrors;

#[derive(Debug, Deserialize, Serialize)]
#[allow(clippy::module_name_repetitions)]
pub struct ModelValidation {
    pub code: String,
    pub message: Option<String>,
}

#[derive(thiserror::Error, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum ModelError {
    #[error("Entity already exists")]
    EntityAlreadyExists,

    #[error("Entity not found")]
    EntityNotFound,

    #[error("{errors:?}")]
    ModelValidation { errors: ModelValidation },

    #[error(transparent)]
    ModelValidationErrors(#[from] ModelValidationErrors),

    #[error("jwt error")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),

    #[error(transparent)]
    Any(#[from] Box<dyn std::error::Error + Send + Sync>),
}

#[allow(clippy::module_name_repetitions)]
pub type ModelResult<T, E = ModelError> = std::result::Result<T, E>;

#[async_trait]
pub trait Authenticable: Clone {
    async fn find_by_api_key(db: &PgPool, api_key: &str) -> ModelResult<Self>;
    async fn find_by_claims_key(db: &PgPool, claims_key: &str) -> ModelResult<Self>;
}
