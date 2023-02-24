use std::sync::Arc;
use async_trait::async_trait;
use crate::Result;

/// ## 18-5 アプリケーションの構成
/// ### リスト18-26 コネクションプール取得トレイト
#[async_trait]
pub trait Pool<T>{
    async fn get() -> Result<Arc<T>>;
}