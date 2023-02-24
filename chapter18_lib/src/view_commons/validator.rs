use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use anyhow::Result;
use thiserror::Error;

///
/// 入力値検証エラー
///
#[derive(Debug , Error)]
pub struct ValidationError {
    pub errors: HashMap<String , String>
}
impl Display for ValidationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f , "{}" , self)
    }
}
impl From<HashMap<String , String>> for ValidationError {
    fn from(_errors: HashMap<String, String>) -> Self {
        Self{errors:_errors}
    }
}

///
/// 入力値検証
///
pub trait AppValidator {
    fn validate_value(&self) -> Result<() , ValidationError>;
}