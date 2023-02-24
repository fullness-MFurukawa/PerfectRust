use serde::Serialize;
use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;

///
/// 値オブジェクト
/// CategoryName構造体
/// 
#[derive(PartialEq, Eq , Clone , Debug , Serialize)]
pub struct CategoryName(String);
impl ValueObject for CategoryName{}       // ValueObjectトレイトの実装
impl TryFrom<String> for CategoryName {   // TryFromトレイトの実装
    type Error = AppError;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(AppError::from("カテゴリ名が存在しません。".to_owned()))
        }else if value.chars().count() > 20 {
            Err(AppError::from("カテゴリ名は20文字以内です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<String> for CategoryName {   // TryIntoトレイトの実装
    type Error = AppError;
    fn try_into(self) -> Result<String> {
        Ok(self.0.clone())
    }
}
impl ToString for CategoryName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_from() {
        match CategoryName::try_from(String::from("")){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"カテゴリ名が存在しません。")
        }
        match CategoryName::try_from(String::from("おいうえおかきくけこさしすせそたちちてとな")){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"カテゴリ名は20文字以内です。")
        }
         
        match CategoryName::try_from(String::from("文房具")){
            Ok(value) => {
                let category_name = CategoryName(String::from("文房具")); 
                assert_eq!(value , category_name);
             } ,
            Err(_) => panic!()
        }
    }
    
    #[test]
    fn try_into() {
        let user_id = CategoryName::try_from(String::from("文房具")).unwrap();
        let value:String = user_id.try_into().unwrap();
        assert_eq!(value , String::from("文房具"));
    }
}