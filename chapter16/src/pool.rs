//!
//! 16章 O/R Mapper サンプルプログラム
//! 

use dotenv::dotenv;
use std::env;
use std::time::Duration;
use anyhow::Result;
use sea_orm::{DatabaseConnection, Database, ConnectOptions};

/// ## 16-2.データベース接続
/// ### リスト16-2コネクションプールの取得
pub struct SamplePool;
impl SamplePool {
    /// ## 16-2.データベース接続
    /// ### リスト16-2 コネクションプールの取得
    pub async fn get() -> Result<DatabaseConnection>{
        dotenv().ok(); // .envファイルを読み取る
        // 接続文字列の取得
        let database_url = env::var("DATABASE_URL")?;
        // 接続プールのオプションを設定する
        let mut options = ConnectOptions::new(database_url);
        options.max_connections(10) // 最大接続数
            .min_connections(5)     // 最小接続数
            .connect_timeout(Duration::from_secs(10))// 接続タイムアウト
            .idle_timeout(Duration::from_secs(10))// アイドルタイムアウト
            .max_lifetime(Duration::from_secs(10))// 最大生存期間
            .sqlx_logging(true); // sqlxのロギング機能を有効にする
        // 接続プールを取得して返す
        let conn = Database::connect(options).await?;
        Ok(conn)
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[tokio::test]
    async fn get_pool_ok() -> Result<()> {
        let result = SamplePool::get().await;
        match result {
            Ok(_) => println!("Ok"),
            Err(error) => println!("{:?}" , error)
        }
        Ok(())
    }
}