

use async_trait::async_trait;
use crate::Result;
use crate::domain::product::product::Product;
use crate::domain::product::product_name::ProductName;

///
/// ## 商品サービス Trait
/// 
#[async_trait]
pub trait ProductService : Send + Sync {
    type Database;
    /// ## キーワード検索
    async fn by_keyword(&self , db:&Self::Database , keyword: &ProductName) -> Result<Vec<Product>>;
    /// ## 新商品の登録
    async fn register(&self , db:&Self::Database , product: &Product) -> Result<Product>;
}
