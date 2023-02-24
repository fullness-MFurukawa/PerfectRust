//!
//! 14章 PostgreSQL
//!

use anyhow::Result;
use tokio_postgres::Transaction;
use async_trait::async_trait;
use crate::section_1::entities::Product;
use crate::section_3::repository::AsyncRepository;

/// ## 14-7.非同期実行
/// ### リスト14-24 AsyncRepositoryトレイトの実装
/// ### productテーブルをアクセスするRepository
pub struct ProductRepository<'a,'b>{
    transaction: &'a mut Transaction<'b>
}
impl<'a,'b> ProductRepository<'a,'b>{
    /// ## 14-7.非同期実行
    /// ### リスト14-24 AsyncRepositoryトレイトの実装
    /// ### インスタンスの生成
    pub fn new(_tran: &'a mut Transaction<'b>) -> Self {
        Self {transaction: _tran}
    }
}
#[async_trait]
impl AsyncRepository<Product , i32 , bool> for ProductRepository<'_,'_> {
    /// ## 14-7.非同期実行
    /// ### リスト14-24 AsyncRepositoryトレイトの実装
    /// ### 全件取得
    async fn select_all(&mut self) -> Result<Vec<Product>> {
        use crate::section_4::sql::get_sql; // 追加
        let sql = get_sql("product", "select_all").await?;// SQL取得関数利用
        let rows = self.transaction.query(sql.as_str() , &[]).await?;
        let mut products = Vec::<Product>::new();
        for row in rows.iter() {
            products.push(Product::new(row.get("id"), row.get("name"),
            row.get("price"), row.get("category_id"), None));
        }
        Ok(products)
    }
}
#[cfg(test)]
pub mod tests{
    use super::*;
    use anyhow::Result;
    use crate::section_3::connect::AsyncSimpleClient;
    use crate::section_3::transaction::AsyncTransactionUtil;
    // select_all()メソッドのテスト
    #[tokio::test]
    async fn select_all() -> Result<()>{
        // データベース接続
        let mut client = AsyncSimpleClient::func_connect().await?;
        // トランザクションの開始
        let mut transaction = AsyncTransactionUtil::
        start(&mut client , true).await?;
        // ProductRepositoryの生成
        let mut repository = ProductRepository::new(&mut transaction);
        // 問合せの実行
        let products = repository.select_all().await?;
        // 結果の出力
        for product in products{
            println!("{:?}" , product.to_string());
        }
        Ok(())
    }
}

