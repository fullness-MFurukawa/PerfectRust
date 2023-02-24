//!
//! 16章 O/R Mapper サンプルプログラム
//!

use sea_orm::entity::*;
use anyhow::{Result,Error};
use async_trait::async_trait;
use sea_orm::{DbBackend,Statement,DatabaseTransaction , EntityTrait, 
              QueryFilter, ColumnTrait, QueryOrder , ConnectionTrait};
use sea_orm::entity::Set;
use crate::entities;
use crate::entities::{product,product_category};
use crate::repository::Repository;
use crate::entities::prelude::{Product,ProductCategory};
use crate::entities::product::Model;

use sea_orm::{EnumIter , DeriveColumn};
/// 結合で取得した結果の別名定義する列挙型
#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
enum ProductCategoryAs {
    CategoryName
}

/// ## 16-4.CRUD操作の準備
/// ### リスト16-6 productテーブルにアクセス構造体
pub struct ProductRepository;
impl  ProductRepository {
    pub fn new() -> Self {
        Self
    }
    /// ### 16-6 テーブル結合
    /// ### リスト16-17 1:1結合
    pub async fn select_by_id_join_product_category(&self ,  tran: &DatabaseTransaction , id: i32)
    -> Result<Vec<(product::Model, Option<product_category::Model>)>>{
        let product_and_category = Product::find_by_id(id)
            .find_also_related(ProductCategory)
            .all(tran).await?;
        Ok(product_and_category)
    }
    
    /// ### 16-7 SQLステートメントの利用
    /// ### リスト16-18 問合せの実装
    pub async fn select_by_id_stmt(&self, tran: &DatabaseTransaction , id: i32) -> Result<Model> {
        // SQLステートメントを生成する
        let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres, // データベースの種類を設定
        "SELECT id,name,price,category_id FROM product WHERE id = $1",// 利用するSQLステートメントを設定
        vec![id.into()]); // プレースホルダにマッピングする値を設定
        // 問合せの実行
        let result:Option<Model> = 
            Product::find().from_raw_sql(stmt)
            .one(tran).await?;
        match result {
            Some(r) => Ok(r),
            None => Err(Error::msg(format!("id:{}に一致する商品が見つかりません。",id)))
        }
    }
    /// ### 16-7 SQLステートメントの利用
    /// ### リスト16-19 更新系SQLの実行
    pub async fn insert_stmt(&self, tran: &DatabaseTransaction , row: Model) -> Result<u64> {
        let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,// データベースの種類を設定
        // 利用するSQLステートメントを設定
        "INSERT INTO product (name,price,category_id) VALUES($1,$2,$3)",
        // プレースホルダにマッピングする値を設定
        vec![row.name.into(),row.price.into(),row.category_id.into()]);
        let result = tran.execute(stmt).await?;// SQLの実行
        Ok(result.rows_affected())
    }
    

}
#[async_trait]
impl Repository for ProductRepository {
    type T = sea_orm::DatabaseTransaction;
    type M = entities::product::Model;
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-8 全件取得
    async fn select_all(&self , tran: &Self::T) -> Result<Vec<Self::M>>{
        let result = Product::find().all(tran).await?;
        Ok(result)
    }
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-10 主キー問合せ
    async fn select_by_id(&self , tran: &Self::T ,id: i32) -> Result<Self::M> {
        let result = 
        Product::find_by_id(id).one(tran).await?;
        match result {
            Some(r) => Ok(r),
            None => Err(Error::msg(format!("id:{}に一致する商品は存在しません。",id)))
        }
    }
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-11 部分一致検索
    async fn select_by_name_like(&self , tran: &Self::T , keyword: &str) -> Result<Vec<Self::M>>{
        let results:Vec<Self::M> = Product::find()
            .filter(product::Column::Name.contains(keyword))
            .order_by_asc(product::Column::Name)
            .all(tran)
            .await?;
        Ok(results) 
    }
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-12 レコードの追加
    async fn insert(&self , tran: &Self::T , row: Self::M) -> Result<Self::M>{
        // ActiveModelを取得する
        let new_product:product::ActiveModel = row.into_active_model();
        let insert_result = new_product.insert(tran).await?;
        Ok(insert_result)
    }
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-14 レコードの更新
    async fn update_by_id(&self , tran: &Self::T , row: Self::M) -> Result<Self::M>{
        // 更新対象を取得する
        let target = 
        Product::find_by_id(row.id).one(tran).await?;
        if target.is_none() {
            return  Err(Error::msg(
            format!("id:{}に一致する商品は存在しません。",row.id)))
        }
        // ActiveModelを取得する
        let mut update_row: product::ActiveModel = target.unwrap().into_active_model();
        // 値を変更する
        update_row.name     =   Set(row.name);
        update_row.price    =   Set(row.price);
        update_row.category_id  =   Set(row.category_id);
        // レコードを更新する
        let update_result = update_row.update(tran).await?;
        Ok(update_result)
    }
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-15 レコードの削除
    async fn delete_by_id(&self , tran: &Self::T , id: i32) -> Result<u64>{
        // 削除対象を取得する
        let target = Product::find_by_id(id).one(tran).await?;
        if target.is_none() {
            return  Err(Error::msg(
            format!("id:{}に一致する商品は存在しません。",id)))
        }
        // ActiveModelを取得する
        let delete_row: product::ActiveModel = target.unwrap().into_active_model();
        // 対象レコードを削除する
        let delete_result = delete_row.delete(tran).await?;
        Ok(delete_result.rows_affected)
    }
} 
#[cfg(test)]
mod tests{
    use super::*;
    use sea_orm::TransactionTrait;
    use crate::pool::SamplePool;
    use crate::entities::product::Model;
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-9 select_all()メソッドのテスト 
    #[tokio::test]
    async fn select_all() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        // すべてのレコードを取得する
        let results = repository.select_all(&transaction).await?;
        for result in results { // 結果を出力する
            println!("{:?}", result);
        }
        Ok(())
    }
    #[tokio::test]
    async fn select_by_id() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        
        let result = repository.select_by_id(&transaction,1).await?;
        println!("{:?}", result);
        let result = repository.select_by_id(&transaction,100).await?;
        println!("{:?}", result);
        Ok(())
    }
    #[tokio::test]
    async fn select_by_name_like() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        
        let results = repository.select_by_name_like(&transaction,"ペン").await?;
        for result in results { // 結果を出力する
            println!("{:?}", result);
        }
        Ok(())
    }
    /// ## 16-5.CRUD操作の実装
    /// ### リスト16-13 insert()メソッドのテスト 
    #[tokio::test]
    async fn insert() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        // 追加するデータを生成する
        let new_product = Model{id:0,name:Some("商品-LMNO".to_owned()),price:Some(200),category_id:Some(1)};
        // レコードを追加する
        let result = repository.insert(&transaction , new_product).await?;
        println!("{:?}" , result);
        transaction.rollback().await?; //  トランザクションをロールバックする
        Ok(())
    }
    #[tokio::test]
    async fn update_by_id() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        // 変更するデータを生成する
        let update_product = Model{id:20,name:Some(
        "商品-LMNO".to_owned()),price:Some(200),category_id:Some(1)};
        // レコードを追加する
        let result = repository.update_by_id(&transaction , update_product).await?;
        println!("{:?}" , result);
        //  トランザクションをロールバックする
        transaction.rollback().await?;
        Ok(())
    }
    #[tokio::test]
    async fn delete_by_id() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        // 変更するデータを生成する
        // レコードを削除する
        let result = repository.delete_by_id(&transaction , 20).await?;
        println!("{:?}" , result);
        //  トランザクションをロールバックする
        transaction.rollback().await?;
        Ok(())
    }
    #[tokio::test]
    async fn select_by_id_join_product_category() -> Result<()> {
         // デバッグログを出力する
         env_logger::builder().filter_level(log::LevelFilter::Debug).init();
         // 接続プールを取得する
         let pool = SamplePool::get().await?;
         // トランザクションを開始する
         let transaction = pool.begin().await?;
         // ProductRepositoryを生成する
         let repository = ProductRepository::new();
        
         let result = repository.select_by_id_join_product_category(&transaction, 1).await?;
         println!("{:?}" , result);
         Ok(())
    }
    #[tokio::test]
    async fn select_by_id_stmt() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        
        let result = repository.select_by_id_stmt(&transaction,10).await?;
        println!("{:?}", result);
        Ok(())
    }
    #[tokio::test]
    async fn insert_stmt() -> Result<()> {
        // デバッグログを出力する
        env_logger::builder().filter_level(log::LevelFilter::Debug).init();
        // 接続プールを取得する
        let pool = SamplePool::get().await?;
        // トランザクションを開始する
        let transaction = pool.begin().await?;
        // ProductRepositoryを生成する
        let repository = ProductRepository::new();
        // 追加するデータを生成する
        let new_product = Model{id:0,name:Some("商品-LMNO".to_owned()),price:Some(200),category_id:Some(1)};
        // レコードを追加する
        let result = repository.insert_stmt(&transaction , new_product).await?;
        println!("{:?}" , result);
        transaction.rollback().await?; //  トランザクションをロールバックする
        Ok(())
    }
}
