//!
//! 16章 O/R Mapper サンプルプログラム
//!

use anyhow::Result;
use async_trait::async_trait;
/// ## 16-4.CRUD操作の準備
/// ### リスト16-6 Repositoryトレイト
#[async_trait]
pub trait Repository {
    type T; // トランザクション型
    type M; // モデル型
    // 全件取得する
    async fn select_all(&self , tran: &Self::T) -> Result<Vec<Self::M>>;
    // 引数に一致する行を取得する
    async fn select_by_id(&self , tran: &Self::T ,id: i32) -> Result<Self::M>;
    // 引数で部分一致検索する
    async fn select_by_name_like(&self , tran: &Self::T , keyword: &str) -> Result<Vec<Self::M>>;
    // 新しいレコードを追加する
    async fn insert(&self , tran: &Self::T , row: Self::M) -> Result<Self::M>;
    // 引数に一致する行の値を変更する
    async fn update_by_id(&self , tran: &Self::T , row: Self::M) -> Result<Self::M>;
    // 引数に一致する行を削除する
    async fn delete_by_id(&self , tran: &Self::T , id: i32) -> Result<u64>;
}
