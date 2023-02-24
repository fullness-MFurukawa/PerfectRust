
use async_trait::async_trait;
use crate::Result;
use crate::domain::product::product::Product;
use crate::domain::product::product_name::ProductName;
///
/// ## 商品 Repository
/// 
#[async_trait]
pub trait ProductRepository: Send + Sync  {
    type Transaction;
    /// 商品キーワード検索する
    async fn select_by_name_like(&self , _: &Self::Transaction , keyword: &ProductName) -> Result<Vec<Product>>;
    /// 新しい商品を永続化する
    async fn insert(&self , _: &Self::Transaction , product: &Product) -> Result<Product>;
    /// 商品名の存在チェックする
    async fn exists(&self , _:&Self::Transaction , name: &ProductName) -> Result<bool>;
}