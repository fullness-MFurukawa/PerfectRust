
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use chapter18_lib::infrastructure::sea_orm::pool::PoolSeaOrm;
use chapter18_lib::infrastructure::pool::Pool;

///
/// DatabaseConnectionの取得
/// 
pub async fn get_db() -> Arc<DatabaseConnection>{
    let pool = PoolSeaOrm::get().await.unwrap();
    pool
}
