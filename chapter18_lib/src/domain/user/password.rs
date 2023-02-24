

use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;

/// ## 18章 外部クレート活用
/// ### ユーザー名を表す値オブジェクト
#[derive(PartialEq, Eq , Clone , Debug)]
pub struct Password(String);
impl ValueObject for Password{} // ValueObjectトレイトの実装
impl TryFrom<String> for Password {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(AppError::from("パスワードが存在しません。".to_owned()))
        }else if value.chars().count() != 128 {
            Err(AppError::from("パスワードは128文字です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<String> for Password {
    type Error = AppError;
    fn try_into(self) -> Result<String> {
        Ok(self.0.clone())
    }
}
impl ToString for Password {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}