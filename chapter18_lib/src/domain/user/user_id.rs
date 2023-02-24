///
/// 18章 外部クレート活用
/// 

use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;

/// ## 18-5 アプリケーションの構成
/// ### リスト18-22 UserId構造体
#[derive(PartialEq, Eq , Clone , Debug)]
pub struct UserId(String);
impl ValueObject for UserId{}       // ValueObjectトレイトの実装
impl TryFrom<String> for UserId {   // TryFromトレイトの実装
    type Error = AppError;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(AppError::from("ユーザーIdが存在しません。".to_owned()))
        }else if value.chars().count() != 36 {
            Err(AppError::from("ユーザーIdは36文字です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<String> for UserId {   // TryIntoトレイトの実装
    type Error = AppError;
    fn try_into(self) -> Result<String> {
        Ok(self.0.clone())
    }
}
impl ToString for UserId {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_from() {
        match UserId::try_from(String::from("")){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"ユーザーIdが存在しません。")
        }
        match UserId::try_from(String::from("ABCD")){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"ユーザーIdは36文字でなければなりません。")
        }
        match UserId::try_from(String::from("5772a800-fef1-40bf-888b-68fddd29d881")){
            Ok(value) => {
                let user_id = UserId(String::from("5772a800-fef1-40bf-888b-68fddd29d881")); 
                assert_eq!(value , user_id);
             } ,
            Err(_) => panic!()
        }
    }
    
    #[test]
    fn try_into() {
        let user_id = UserId::try_from(String::from("5772a800-fef1-40bf-888b-68fddd29d881")).unwrap();
        let value:String = user_id.try_into().unwrap();
        assert_eq!(value , String::from("5772a800-fef1-40bf-888b-68fddd29d881"));
    }
}