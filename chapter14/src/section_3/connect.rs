//!
//! 14章 PostgreSQL
//!

use anyhow::Result;
use tokio_postgres::{Client, NoTls};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::section_1::params::ConnectParams;
// データベース接続情報
pub static CONNECT_PARAMS: Lazy<Mutex<ConnectParams>> = 
Lazy::new( || {
    let params = ConnectParams::new(
        "localhost".to_owned() ,  5432,  "rust_sample".to_owned() ,
        "postgres".to_owned(),  "postgres".to_owned());
    Mutex::new(params)
});
/// ## 14-7.非同期処理
/// ### リスト14-21 データベース接続機能の実装
pub struct AsyncSimpleClient;
impl AsyncSimpleClient {
    /// ## データベース接続
    /// ### リスト14-21 データベース接続機能の実装
    pub async fn func_connect() -> Result<Client> {
        let params = CONNECT_PARAMS.lock().unwrap();
        // 接続要求を出す
        let (client , connection) =
        tokio_postgres::connect(
        params.connect_string().as_str(), NoTls).await?;
        tokio::spawn(async move {  // 非同期タスクで接続完了通知を待つ
            if let Err(e) = connection.await {
                panic!("接続エラー: {}", e);
            }
        });
        Ok(client)
    }
}

