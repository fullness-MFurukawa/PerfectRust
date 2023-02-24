pub mod handler;

use crate::handler::error::ApiAppError;
pub type Result<T> = anyhow::Result<T , ApiAppError>;