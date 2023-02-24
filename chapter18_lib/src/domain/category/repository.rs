
use async_trait::async_trait;
use crate::Result;
use crate::domain::category::category::Category;
use crate::domain::category::category_id::CategoryId;
///
///  ## 商品カテゴリ Repository
/// 
#[async_trait]
pub trait CategoryRepository : Send + Sync {
    type Transaction;
    ///　すべてのカテゴリを取得する
    async fn select_all(&self , _: &Self::Transaction) -> Result<Vec<Category>>;
    ///　指定された識別子でカテゴリを取得する
    async fn select_by_id(&self , _: &Self::Transaction , id: &CategoryId) -> Result<Option<Category>>;
}