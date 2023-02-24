use serde::Serialize;
use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;

//
/// 値オブジェクト
/// ProductId構造体
/// 
#[derive(PartialEq, Eq , Clone , Debug , Serialize)]
pub struct ProductId(i32);
impl ValueObject for ProductId{}    // ValueObjectトレイトの実装
impl TryFrom<i32> for ProductId {   // TryFromトレイトの実装
    type Error = AppError;
    fn try_from(value: i32) -> Result<Self> {
        if value < 0 {
            Err(AppError::from("商品IDが不正です。".to_owned()))
        }else{
            Ok(Self(value))
        }
    }
}
impl TryInto<i32> for ProductId {   // TryIntoトレイトの実装
    type Error = AppError;
    fn try_into(self) -> Result<i32> {
        Ok(self.0.clone())
    }
}
impl ToString for ProductId{
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn try_from() {
        match ProductId::try_from(-1){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"商品IDが不正です。")
        }
        match ProductId::try_from(1){
            Ok(category_id) => assert_eq!(1 , category_id.0) ,
            Err(_) => panic!() 
        }
    }
    #[test]
    fn try_into() {
        let product_id = ProductId::try_from(100).unwrap();
        let value:i32 = product_id.try_into().unwrap();
        assert_eq!(100 , value);
    }
}