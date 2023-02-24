//!
//! 18章 外部クレート活用
//! 

use std::sync::Arc;
use std::env;
use std::time::Duration;
use dotenv::dotenv;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection,ConnectOptions,Database};
use crate::Result;
use crate::error::AppError;
use crate::infrastructure::pool::Pool;
/// ## 18-5 アプリケーションの構成
/// ### リスト18-26 SeaORMのコネクションプール取得
pub struct PoolSeaOrm;
#[async_trait]
impl Pool<DatabaseConnection> for PoolSeaOrm{
    async fn get() -> Result<Arc<DatabaseConnection>>{
        dotenv().expect(".envファイルが見つかりません。");
        let database_url = env::var("DATABASE_URL").expect("DATABSE_URLが取得できません。");
        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);
        match Database::connect(opt).await{
            Ok(connection) => Ok(Arc::new(connection)) ,
            Err(error) => Err(AppError::from(error))
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[tokio::test]
    async fn get() {
        let pool = PoolSeaOrm::get().await.unwrap();
        println!("{:?}",pool);
        assert!(true);
    }
}

