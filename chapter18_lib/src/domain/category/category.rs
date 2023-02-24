use serde::Serialize;
use crate::domain::category::category_id::CategoryId;
use crate::domain::category::category_name::CategoryName;
use crate::domain::entity::Entity;
use crate::Result;

///
/// ## 商品カテゴリ Entity
/// 
#[derive(PartialEq , Eq , Clone , Debug , Serialize)]
pub struct Category {
    pub category_id: CategoryId     , // カテゴリID
    pub category_name: CategoryName   // カテゴリ名
}
impl Category { 
    /// ## Categoryの構築
    pub fn new(_category_id: CategoryId , _category_name: CategoryName) -> Result<Self>{
        Ok(Self {category_id:_category_id , category_name:_category_name})
    }
    /// ## Categoryの構築
    /// 値オブジェクトを生成し、Categoryを構築する
    pub fn build(_category_id: i32 , _category_name: String) -> Result<Self>{
        Ok(Self { category_id: CategoryId::try_from(_category_id)?, 
            category_name: CategoryName::try_from(_category_name)? })
    }
}
impl Entity for Category {}