//!
//! 15章 MongoDBサンプルコード
//! 

use anyhow::{Result, Error};
use mongodb::Collection;
use async_trait::async_trait;
use futures_util::StreamExt;
use mongodb::options::{FindOptions,UpdateModifications};
use mongodb::bson::doc;
use crate::entities::Product;
use crate::connect::SampleMongoClient;
use crate::repository::Repository;

/// productコレクションを操作するRepository
pub struct ProductRepository{
    collection: Collection<Product> // Product Collection
}
impl ProductRepository {
    // 値を生成する
    pub fn new(sample_client: SampleMongoClient, collection_name: &str) 
    -> Self {
        Self{ // Collectionをフィールドにセットする
            collection: 
            sample_client.get_database().collection(collection_name)
        }
    }
}
#[async_trait]
impl Repository<Product , i32 , bool> for ProductRepository {
    /// ## 15-4.CRUD操作の実装
    /// ### リスト15-7. 全件取得
    async fn select_all(&self) -> Result<Vec<Product>> {
        // 単価で昇順ソートする
        let find_options = FindOptions::builder()
        .sort(doc! {"price": 1 }).build();
        let mut cursor = self.collection.find(None,
        find_options).await?;
        let mut products = Vec::new();
        // 行を取り出してProductに格納する
        while let Some(product) = cursor.next().await {
            products.push(product?);
        }
        Ok(products)
    }
    /// ## 15-4.CRUD操作の実装
    /// ### リスト15-8パラメータ検索
    async fn select_by_id(&self, id: i32) -> Result<Product> {
        // 検索パラメータを設定する
        let filter = doc! {"id": id };
        // 対象のDocumentが存在しない場合、エラーメッセージを返す
        let x = self.collection.find_one(filter , None).await?;
        self.collection.find_one(filter , None).await?
            .ok_or(Error::msg(format!("商品id:{}は存在しません。" , id)))
    }
        /// ## 15-4.CRUD操作の実装
        /// #### リスト15-9 指定された商品を追加する
        async fn insert(&self, product: Product) -> Result<bool> {
            // 引数のDocumentを追加する
            self.collection.insert_one(product,None)
                .await.map(|_| Ok(true))?
        }
        /// ## 15-4.CRUD操作の実装
        //  #### リスト15-9 指定された複数の商品を追加する
        async fn insert_many(&self,products: Vec<Product>) -> Result<bool> {
            // Vecに格納されたDcumentを追加する
            self.collection.insert_many(products.clone(),None)
                .await.map(|ret|{
                // Vecの件数と追加された件数が同じであればtrueを返す
                if ret.inserted_ids.iter().count() == products.iter().count() {
                    Ok(true)
                }else{
                    Ok(false)
                }
            })?
        }
    /// ## 15-4.CRUD操作の実装
    /// #### リスト15-10 指定された値に変更する
    async fn update_by_id(&self, row: Product) -> Result<bool> {
        // 更新対象取得条件のDocumentを取得する
        let query = doc! {"product_id": row.get_id() };
        // 変更項目と変更する値のDocumentを生成する
        let update = UpdateModifications::Document(
            //商品名と単価を変更する
            doc! {"$set": {"name": row.get_name(),"price": row.get_price()}}
        );
        // Documentの値を変更する
        let result = self.collection.update_one(query,update,None)
            .await.map(|result|{
            //変更件数が1ならtrueを返す
            if result.modified_count == 1 { true }
            else{ false }
        })?;
        Ok(result)
    }
    /// ## 15-4.CRUD操作の実装
    //  ### リスト15-11 指定されたidのドキュメントを削除する
    async fn delete_by_id(&self, id: i32) -> Result<bool> {
        let filter = doc! {"product_id": id };
        let result = self.collection.delete_one(filter,None)
            .await.map(|result|{
            //　削除件数が1ならtrueを返す
            if result.deleted_count == 1{ true }
            else{ false }
        })?;
        Ok(result)
    }
   
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::shared::setup;
    use crate::entity::product_category::ProductCategory;
}

