//!
//! 14章 PostgreSQL
//!

use r2d2_postgres::{postgres::NoTls, PostgresConnectionManager};
use r2d2::PooledConnection;
use anyhow::Result;
use crate::section_2::pool_1::SAMPLE_POOL_1;

/// ## 14-6.コネクションプール
/// ### リスト14-16 コネクションの取得
/// ### コネクションマネージャ
#[derive(Debug)]
pub struct SamplePoolManager;
impl SamplePoolManager {
    /// ## 14-6.コネクションプール
    /// ### リスト14-16 コネクションプールから接続を取得する
    pub fn client() -> Result<PooledConnection<PostgresConnectionManager<NoTls>>> {
        let pool = SAMPLE_POOL_1.lock().unwrap(); // ロックを解除してプールを取得する
        println!("state {:?}" , pool.state());  // 状態を出力
        let client = pool.get()?;               // コネクションプールから接続を取得する
        Ok(client)
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use std::thread;
    use crate::section_1::repository::Repository;
    use crate::section_1::transaction::TransactionUtil;
    use crate::section_1::product_repository::ProductRepository;
    #[test]
    fn use_connection_pool() -> Result<()>{
        let mut handles = Vec::with_capacity(3);
        for num in 0..3 {
            handles.push(thread::spawn(move || {
                let mut client = SamplePoolManager::client()?;
                let mut transaction = 
                TransactionUtil::start(&mut client, true)?;
                let mut repository = ProductRepository(&mut transaction);
                repository.select_by_id(num+1)
            }));
        }
        for handle in handles{
            let result = handle.join().unwrap();
            println!("{}" , result.unwrap().to_string());
        }
        Ok(())
    }
}