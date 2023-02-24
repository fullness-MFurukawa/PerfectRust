//!
//! 14章 PostgreSQL
//! 

use std::sync::Mutex;
use std::time::Duration;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2_postgres::r2d2::Pool;
use once_cell::sync::Lazy;
// 接続文字列
#[allow(dead_code)]
static CONNECT_STR: &str = "host=localhost port=5432 dbname=rust_sample user=postgres password=postgres";
/// コネクションプールを生成する
#[allow(dead_code)]
pub static SAMPLE_POOL_2: Lazy<Mutex<Pool<PostgresConnectionManager<NoTls>>>> =
Lazy::new(|| {
    let config = CONNECT_STR.parse().unwrap(); // 接続設定を生成する
    let manager = PostgresConnectionManager::new(config, NoTls);
    let pool = r2d2::Pool::builder()
        .max_size(30) // 最大接続数
        .min_idle(Some(1)) //最小アイドル接続数
        .connection_timeout(Duration::from_secs_f32(60.0)) //タイムアウト時間
        .build(manager).unwrap();// コネクションプールの生成
    Mutex::new(pool)
});
