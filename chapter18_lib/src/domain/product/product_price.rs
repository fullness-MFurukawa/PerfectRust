use serde::Serialize;
use crate::{AppError,Result};
use crate::domain::value_object::ValueObject;
///
/// 値オブジェクト
/// ProductPrice構造体
/// 
#[derive(PartialEq, Eq , Clone , Debug , Serialize)]
pub struct ProductPrice(i32);
impl ValueObject for ProductPrice{}    // ValueObjectトレイトの実装
impl TryFrom<i32> for ProductPrice {   // TryFromトレイトの実装
    type Error = AppError;
    fn try_from(value: i32) -> Result<Self> {
        if value >= 50 && value <= 10000 {
            Ok(Self(value))
        }else{
            Err(AppError::from("単価が不正です。".to_owned()))
        }
    }
}
impl TryInto<i32> for ProductPrice {   // TryIntoトレイトの実装
    type Error = AppError;
    fn try_into(self) -> Result<i32> {
        Ok(self.0.clone())
    }
}
impl ToString for ProductPrice {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}
#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn try_from() {
        match ProductPrice::try_from(0){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"単価が不正です。")
        }
        match ProductPrice::try_from(49){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"単価が不正です。")
        }
        match ProductPrice::try_from(10001){
            Ok(_) => panic!() ,
            Err(error) => assert_eq!(error.to_string().as_str(),"単価が不正です。")
        }
        match ProductPrice::try_from(50){
            Ok(category_id) => assert_eq!(50 , category_id.0) ,
            Err(_) => panic!() 
        }
        match ProductPrice::try_from(10000){
            Ok(category_id) => assert_eq!(10000 , category_id.0) ,
            Err(_) => panic!() 
        }
    }
    #[test]
    fn try_into() {
        let product_price = ProductPrice::try_from(100).unwrap();
        let value:i32 = product_price.try_into().unwrap();
        assert_eq!(100 , value);
    }
}