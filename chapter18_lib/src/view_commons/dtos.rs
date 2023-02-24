use serde::{Serialize , Deserialize};
use rusty_money::{Money , iso};
use crate::domain::category::category::Category;
use crate::domain::product::product::Product;
use crate::domain::user::user::User;


///
/// ユーザーDTO
/// 
#[derive(Clone , Debug , Serialize , Deserialize)]
pub struct UserDto {
    pub user_id: String ,   // ユーザーId
    pub user_name: String , // ユーザー名
    pub password: String,   // パスワード
    pub mail:     String    // メールアドレス
}
impl UserDto {
    // UserからUserDTOに変換する
    pub fn convert(user: User) -> Self {
        Self { user_id: user.user_id.to_string(), 
               user_name: user.user_name.to_string(), 
               password: user.password.to_string(), 
               mail: user.mail.to_string() }
    }
}
///
/// カテゴリ結果表示用DTO
/// 
#[derive(Clone , Debug , Serialize , Deserialize)]
pub struct CategoryDto {
    pub id: String ,    // カテゴリ番号
    pub name: String ,  // カテゴリ名
}
impl CategoryDto{
    // CategoryからCategoryDTOに変換する
    pub fn convert(category: Category) -> Self {
        Self { id: category.category_id.to_string() , name: category.category_name.to_string() }
    }
    // Vec<Category>からVec<CategoryDTo>に変換する
    pub fn converts(categories: Vec<Category>) -> Vec<CategoryDto> {
        let mut results:Vec<Self> = Vec::new();
        for category in categories {
            results.push(Self::convert(category))
        }
        results
    }
}
///
/// 商品 結果表示用DTO
/// 
#[derive(Clone , Debug , Serialize , Deserialize)]
pub struct ProductDto {
    pub id: String ,    // 商品番号
    pub name: String ,  // 商品名
    pub price: String , // 単価
    pub category_name: String // カテゴリ名
}
impl ProductDto {
    // ProductからProductDTOに変換する
    pub fn convert(product: Product) -> Self {
        let price:i32 = product.product_price.try_into().unwrap();
        let currency = Money::from_major(price as i64, iso::JPY).to_string();
        Self {  id: product.product_id.to_string(), 
                name: product.product_name.to_string(), 
                price: currency, 
                category_name: product.category.unwrap().category_name.to_string() }
    }
    // Vec<Product>からVec<ProductDTo>に変換する
    pub fn converts(products: Vec<Product>) -> Vec<ProductDto> {
        let mut results:Vec<Self> = Vec::new();
        for product in products {
            results.push(Self::convert(product))
        }
        results
    }
}

