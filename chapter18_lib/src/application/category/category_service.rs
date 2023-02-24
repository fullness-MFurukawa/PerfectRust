

use async_trait::async_trait;
use crate::Result;
use crate::domain::category::category::Category;
use crate::domain::category::category_id::CategoryId;

///
/// ## カテゴリサービス Trait
/// 
#[async_trait]
pub trait CategoryService : Send + Sync {
    type Database;
    /// ## 全件取得する
    async fn categories(&self , _:&Self::Database) -> Result<Vec<Category>>;
    /// ## 指定されたidのカテゴリを取得する
    async fn category(&self , _:&Self::Database , id: &CategoryId) -> Result<Category>;
}