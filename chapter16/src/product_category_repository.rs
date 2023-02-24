//!
//! 16章 O/R Mapper サンプルプログラム
//!

use anyhow::Result;
use sea_orm::{DatabaseTransaction ,EntityTrait};
use crate::entities::{product,product_category};
use crate::entities::prelude::{Product,ProductCategory};

/// ## 16-4.CRUD操作の準備
/// ### リスト16-6 productテーブルにアクセス構造体
pub struct ProductCategoryRepository;
impl  ProductCategoryRepository {
    pub fn new() -> Self {
        Self
    }
    /// ### 16-6 テーブル結合
    /// ### リスト16-16 1:N結合
    pub async fn select_by_id_join_product(&self , tran: &DatabaseTransaction , id: i32)
    -> Result<Vec<(product_category::Model, Vec<product::Model>)>>{
        let category_and_product =
            ProductCategory::find_by_id(id)
            .find_with_related(Product)
            .all(tran).await?;
        Ok(category_and_product)
    }

}
#[cfg(test)]
mod tests{
    use super::*;
    use sea_orm::TransactionTrait;
    use crate::pool::SamplePool;
    /// 1:N結合のテスト
    #[tokio::test]
    async fn select_by_id_join_product() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductCategoryRepository::new();
        // すべてのレコードを取得する
        let result = 
            repository.select_by_id_join_product(&transaction, 1).await;
        println!("{:?}", result);
        Ok(())
    }
}
