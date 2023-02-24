
use serde::Serialize;
use crate::domain::entity::Entity;
use crate::domain::product::product_id::ProductId;
use crate::domain::product::product_price::ProductPrice;
use crate::domain::category::category::Category;
use super::product_name::ProductName;
use crate::Result;

///
/// ## 商品 Entity
/// 
#[derive(PartialEq , Eq , Clone , Debug , Serialize )]
pub struct Product {
    pub product_id:   ProductId ,   // 商品ID
    pub product_name: ProductName , // 商品名
    pub product_price:ProductPrice, // 商品単価
    pub category: Option<Category>  // カテゴリ
}
impl Product{
    /// ## Productの構築
    pub fn new(_product_id: ProductId , _product_name: ProductName ,
    _product_price: ProductPrice , _category: Option<Category>) -> Result<Self>{
        Ok(Self {product_id: _product_id ,
                 product_name: _product_name ,
                 product_price:_product_price , 
                 category: _category})
    }
    /// ## Productの構築
    /// 値オブジェクトを生成し、Productを構築する
    pub fn build(_product_id: i32 , _product_name: String , 
        _product_price: i32 , _category: Option<Category>) -> Result<Self>{
        Ok(Self { product_id: ProductId::try_from(_product_id)?,
                  product_name: ProductName::try_from(_product_name)?,
                  product_price: ProductPrice::try_from(_product_price)?, 
                  category: _category })
    }
}
impl Entity for Product {}