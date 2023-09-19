use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::models::user_model::User;

pub mod user_model;
pub mod auth_model;
pub mod login_model;


#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}
