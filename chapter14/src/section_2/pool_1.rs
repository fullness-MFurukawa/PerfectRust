//!
//! 14章 PostgreSQL
//! 

use std::sync::Mutex;
use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2_postgres::r2d2::Pool;
use once_cell::sync::Lazy;

// 接続文字列
static CONNECT_STR: &str = "host=localhost port=5432 dbname=rust_sample user=postgres password=postgres";
#[allow(dead_code)]
/// ## 14-6.コネクションプール
/// リスト14-16 コネクションプールの生成
pub static SAMPLE_POOL_1: Lazy<Mutex<Pool<PostgresConnectionManager<NoTls>>>> =
Lazy::new(|| {
    // 接続設定を生成する
    let config = CONNECT_STR.parse().unwrap();
    // コネクションマネージャを生成する
    let manager = PostgresConnectionManager::new(config, NoTls);
    // コネクションプールを生成する
    let pool = r2d2::Pool::new(manager).unwrap();
    // コネクションプールをMutexでラップする
    Mutex::new(pool)
});

