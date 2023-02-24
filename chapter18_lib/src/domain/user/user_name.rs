
use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;

/// ## 18章 外部クレート活用
/// ### ユーザー名を表す値オブジェクト
#[derive(PartialEq, Eq , Clone , Debug)]
pub struct UserName(String);
impl ValueObject for UserName{} // ValueObjectトレイトの実装
impl TryFrom<String> for UserName {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(AppError::from("ユーザー名が存在しません。".to_owned()))
        }else if value.chars().count() <= 6 && value.chars().count() >= 20 {
            Err(AppError::from("ユーザー名は6文字以上、20文字以内です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<String> for UserName {
    type Error = AppError;
    fn try_into(self) -> Result<String> {
        Ok(self.0.clone())
    }
}
impl ToString for UserName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}