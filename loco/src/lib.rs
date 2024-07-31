#![allow(clippy::module_name_repetitions)]

pub use self::errors::Error;

mod banner;
pub mod prelude;

pub mod db;

pub mod model;
mod template;

pub mod app;
#[cfg(feature = "cli")]
pub mod cli;

pub mod auth;
pub mod boot;
pub mod cache;
pub mod config;
pub mod controller;
pub mod environment;
pub mod errors;
pub mod hash;
mod logger;
pub mod mailer;
mod redis;
#[cfg(feature = "testing")]
pub mod testing;
#[cfg(feature = "testing")]
pub use axum_test::TestServer;
pub mod storage;
pub mod validation;
pub mod worker;
#[cfg(feature = "channels")]
pub use socketioxide;
pub use validator;

/// Application results options list
pub type Result<T, E = Error> = std::result::Result<T, E>;
