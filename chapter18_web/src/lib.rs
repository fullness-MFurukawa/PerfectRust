pub mod handler;

use crate::handler::error::WebAppError;
pub type Result<T> = anyhow::Result<T , WebAppError>;