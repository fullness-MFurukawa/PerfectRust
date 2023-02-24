use serde::Serialize;
use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;

///
/// 値オブジェクト
/// ProductName構造体
/// 
#[derive(PartialEq, Eq , Clone , Debug , Serialize)]
pub struct ProductName(String);
impl ValueObject for ProductName{}       // ValueObjectトレイトの実装
impl TryFrom<String> for ProductName {   // TryFromトレイトの実装
    type Error = AppError;
    fn try_from(value: String) -> Result<Self> {
        if value.is_empty() {
            Err(AppError::from("商品名が存在しません。".to_owned()))
        }else if value.chars().count() > 30 {
            Err(AppError::from("商品名は30文字以内です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<String> for ProductName {   // TryIntoトレイトの実装
    type Error = AppError;
    fn try_into(self) -> Result<String> {
        Ok(self.0.clone())
    }
}
impl ToString for ProductName {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_from() {
        match ProductName::try_from(String::from("")){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"商品名が存在しません。")
        }
        match ProductName::try_from(String::from("おいうえおかきくけこさしすせそたちちてとなにぬねのはひふへほわ")){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"商品名は30文字以内です。")
        }
        match ProductName::try_from(String::from("商品-A")){
            Ok(value) => {
                let product_name = ProductName(String::from("商品-A")); 
                assert_eq!(value , product_name);
             } ,
            Err(_) => panic!()
        }
    }
    #[test]
    fn try_into() {
        let product_name = ProductName::try_from(String::from("商品-A")).unwrap();
        let value:String = product_name.try_into().unwrap();
        assert_eq!(value , String::from("商品-A"));
    }
}