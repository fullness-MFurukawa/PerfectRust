
use serde::Serialize;
use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;

///
/// 値オブジェクト
/// CategoryId構造体
/// 
#[derive(PartialEq, Eq , Clone , Debug , Serialize)]
pub struct CategoryId(i32);
impl ValueObject for CategoryId{}    // ValueObjectトレイトの実装
impl TryFrom<i32> for CategoryId {   // TryFromトレイトの実装
    type Error = AppError;
    fn try_from(value: i32) -> Result<Self> {
        if value < 1 {
            Err(AppError::from("カテゴリIDが不正です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<i32> for CategoryId {   // TryIntoトレイトの実装
    type Error = AppError;
    fn try_into(self) -> Result<i32> {
        Ok(self.0.clone())
    }
}
impl ToString for CategoryId {
   fn to_string(&self) -> String {
       self.0.to_string()
   } 
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn try_from() {
        match CategoryId::try_from(0){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"カテゴリIDが不正です。")
        }
        match CategoryId::try_from(-1){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"カテゴリIDが不正です。")
        }
        match CategoryId::try_from(1){
            Ok(category_id) => assert_eq!(1 , category_id.0) ,
            Err(_) => panic!() 
        }
    }
    #[test]
    fn try_into() {
        let category_id = CategoryId::try_from(100).unwrap();
        let value:i32 = category_id.try_into().unwrap();
        assert_eq!(100 , value);
    }
}