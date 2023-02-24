//!
//! 15章 MongoDBサンプルコード
//! 

use anyhow::Result;
use async_trait::async_trait;

/// CRUD操作トレイト
#[async_trait]
pub trait Repository<T , K , UPD> {
    // すべてのそきゅメントを取得する
    async fn select_all(&self) -> Result<Vec<T>> ; 
    // 引数に一致するドキュメントを取得する
    async fn select_by_id(&self , id: K) -> Result<T>;
    // ドキュメントを追加する
    async fn insert(&self , row: T) -> Result<UPD>;
    // 複数のドキュメントを追加する
    async fn insert_many(&self , rows: Vec<T>) -> Result<UPD>;
    // 引数指定されたドキュメントを変更する
    async fn update_by_id(&self , row: T) -> Result<UPD>;
    // 引数指定されたドキュメントを削除する
    async fn delete_by_id(&self , id: K) -> Result<UPD>;
    /* 
    // ドキュメントの件数を取得する
    async fn count_documents(&self) -> Result<u64> ;
    */
}


