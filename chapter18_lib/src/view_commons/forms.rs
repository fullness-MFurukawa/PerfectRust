
use std::collections::HashMap;
use serde::{de, Deserialize , Serialize};
use validator::{validate_length , validate_required , validate_range , Validate};
use crate::domain::category::{category::Category,category_id::CategoryId,category_name::CategoryName};
use crate::domain::product::{product::Product,product_id::ProductId, product_name::ProductName,product_price::ProductPrice};
use crate::domain::user::user::User;
use crate::view_commons::validator::{AppValidator,ValidationError};

///
/// 空の文字列をNoneにマッピングする
///
pub fn empty_string_as_none<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
    where T: de::Deserialize<'de>, D: de::Deserializer<'de> {
    Ok(T::deserialize(deserializer).ok())
}

///
/// キーワード検索用DTO
/// 
#[derive(Deserialize , Debug)]
pub struct ProductSearchForm {
    pub keyword: Option<String>
}
/// 入力値検証
impl AppValidator for ProductSearchForm{
    fn validate_value(&self) -> anyhow::Result<(), ValidationError> {
        let mut errors:HashMap<String,String> = HashMap::new();
        // 未入力と範囲チェック
        if self.keyword.is_none() || self.keyword.as_ref().unwrap().is_empty() {
            errors.insert(String::from("keyword"),String::from("キーワードは入力必須です。"));
        }
        if errors.is_empty(){
            Ok(())
        }else{
            Err(ValidationError::from(errors))
        }
    }
}
impl Into<ProductName> for ProductSearchForm {
    fn into(self) -> ProductName {
        ProductName::try_from(self.keyword.unwrap().clone()).unwrap()
    }
}

///
/// 新商品登録用DTO
/// 
#[derive(Deserialize , Serialize , Debug , Clone)]
pub struct ProductRegisterForm {
    pub name:           Option<String> ,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub price:          Option<i32> ,
    #[serde(deserialize_with = "empty_string_as_none")]
    pub category_id:    Option<i32>
}
/// 入力値検証
impl AppValidator for ProductRegisterForm{
    fn validate_value(&self) -> anyhow::Result<(), ValidationError> {
        let mut errors:HashMap<String,String> = HashMap::new();
        // nameフィールドの検証 未入力と文字数チェック
        if ! validate_length(self.name.as_ref().unwrap(), Some(4), Some(20), None) {
            errors.insert(String::from("name"),String::from("商品名は４文字以上20文字以内で入力して下さい。"));
        }
        // priceフィールドの検証　未入力と範囲チェック
        if ! validate_required(&self.price) {
            errors.insert(String::from("price"),String::from("単価は入力必須です。"));
        }else{
            if ! validate_range(self.price.unwrap(), Some(50), Some(100000) ){
                errors.insert(String::from("price") , String::from("単価は50～100000までで入力して下さい。"));
            }
        }
        // category_idフィールドの検証　未入力と範囲チェック
        if ! validate_required(&self.category_id) {
            errors.insert(String::from("category_id"),String::from("カテゴリは入力必須です。"));
        }else{
            if ! validate_range(self.category_id.unwrap(), Some(1), Some(3) ){
                errors.insert(String::from("category_id"),String::from("不正なカテゴリが選択されました。"));
            }
        }
        if errors.is_empty(){
            Ok(())
        }else{
            Err(ValidationError::from(errors))
        }
    }
}
impl Into<Product> for ProductRegisterForm {
    fn into(self) -> Product {
        let product_id = ProductId::try_from(0).unwrap();
        let product_name = ProductName::try_from(self.name.unwrap().clone()).unwrap();
        let product_price = ProductPrice::try_from(self.price.unwrap().clone()).unwrap();
        let category_id = CategoryId::try_from(self.category_id.unwrap().clone()).unwrap();
        let category_name = CategoryName::try_from(String::from("dummy")).unwrap();
        let category = Category::new(category_id,category_name).unwrap();
        let product = Product::new(product_id, product_name, product_price, Some(category)).unwrap();
        product
    }
}

///
/// 認証用DTO
/// 
#[derive(Debug , Clone , Deserialize , Serialize , Validate)]
pub struct LoginForm {
    #[validate(length(min = 6 , max = 20, message="ユーザー名は6文字以上20文字以内で入力して下さい。"))]
    pub name:       Option<String> , //  ユーザー名
    #[validate(length(min = 6 , max = 20, message="パスワードは6文字以上20文字以内で入力して下さい。"))]
    pub password:   Option<String>   //  パスワード
}
/// 入力値検証
impl AppValidator for LoginForm {
    fn validate_value(&self) -> anyhow::Result<(), ValidationError> {
        let mut errors:HashMap<String,String> = HashMap::new();
        match self.validate() {
            Ok(_) => Ok(()) ,
            Err(validate_errors) => {
                let field_errors = validate_errors.field_errors();
                let error =  field_errors.get("name");
                if error.is_some(){
                    let name_errors = error.unwrap();
                    for name_error in *name_errors {
                        errors.insert("name".to_string(), name_error.message.as_ref().unwrap().to_string());
                    }
                }
                let error =  field_errors.get("password");
                if error.is_some() {
                    let password_errors = error.unwrap();
                    for password_error in *password_errors {
                        errors.insert("password".to_string(), password_error.message.as_ref().unwrap().to_string());
                    }
                }
                Err(ValidationError::from(errors))
            }
        }
    }
}
impl Into<User> for LoginForm {
    fn into(self) -> User {
        let _mail = String::from("dummy@dummy.com");
        let user:User = User::new( self.name.unwrap(), 
                                    self.password.unwrap(), 
                                    _mail).unwrap();
        user 
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn user_into(){
        let login_form = LoginForm{name: Some(String::from("user001")),
                                    password: Some(String::from("pass001"))};
        let user:User = login_form.into();
        println!("{:?}" , &user);
        assert!(true);
    }
}

