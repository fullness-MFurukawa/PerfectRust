


use crate::{AppError , Result};
use crate::domain::value_object::ValueObject;

/// ## 18章 外部クレート活用
/// ### ユーザー名を表す値オブジェクト
#[derive(PartialEq, Eq , Clone , Debug)]
pub struct Mail(String);
impl ValueObject for Mail{} // ValueObjectトレイトの実装
impl TryFrom<String> for Mail {
    type Error = AppError;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(AppError::from("メールアドレスが存在しません。".to_owned()))
        }else if value.chars().count() < 10 && value.chars().count() > 50 {
            Err(AppError::from("メールアドレスは10文字以上、50文字以内です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<String> for Mail {
    type Error = AppError;
    fn try_into(self) -> Result<String> {
        Ok(self.0.clone())
    }
}
impl ToString for Mail{
    fn to_string(&self) -> String {
        self.0.clone()
    }
}