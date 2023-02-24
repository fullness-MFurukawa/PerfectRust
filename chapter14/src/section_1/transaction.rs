//!
//! 14章 PostgreSQL
//! 

use postgres::{Client, Transaction};
use anyhow::Result;
/// ## 14-3.トランザクション制御
/// ### リスト14-4 トランザクション制御機能の実装
pub struct TransactionUtil; // トランザクション制御構造体
impl TransactionUtil {
    /// ## トランザクションを開始する
    /// ### 引数 client: データベース接続
    /// ### 引数 mode true:読取専用トランザクション
    /// ### 戻り値 Transaction:開始したトランザクション
    pub fn start<'a>(client: &'a mut Client, mode: bool) -> Result<Transaction<'a>>{
    let transaction = client.build_transaction().read_only(mode).start()?;
        Ok(transaction)
    }
    /// ## トランザクションをコミットする
    /// #### 引数 transaction:コミット対象のトランザクション
    pub fn commit(transaction: Transaction) ->Result<()> {
        transaction.commit()?;
        Ok(())
    }
    /// ## トランザクションをロールバックする
    /// #### 引数 transaction:ロールバック対象のトランザクション
    pub fn rollback(transaction: Transaction) ->Result<()> {
        transaction.rollback()?;
        Ok(())
    }
}  
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use crate::section_1::params::ConnectParams;
    use crate::section_1::connect::PostgresSampleClient;
    fn create_client() -> Result<Client>{
        let params = ConnectParams::new(
                "localhost".to_owned() ,
                5432,
                "rust_sample".to_owned() ,
                "postgres".to_owned(),
                "postgres".to_owned());
        PostgresSampleClient::simple_connect(params)
    }
    #[test]
    fn start_transaction_ok(){
            let mut client = create_client().unwrap();
            let transaction = super::TransactionUtil::start(&mut client , true);
            match transaction {
                Ok(_) => assert!(true),
                Err(error) => {
                    println!("{:?}" , error.to_string());
                    assert!(false);
                }
            }
            let mut client = create_client().unwrap();
            let transaction = super::TransactionUtil::start(&mut client , false);
            match transaction {
                Ok(_) => assert!(true),
                Err(error) => {
                    println!("{:?}" , error.to_string());
                    assert!(false);
                }
            }
    }
    #[test]
    fn commit_ok() -> Result<()>{
            let mut client = create_client()?;
            let transaction = super::TransactionUtil::start(&mut client , false)?;
            let result = transaction.commit();
            match result {
                Ok(_) => assert!(true),
                Err(error) => {
                    println!("{:?}" , error.to_string());
                    assert!(false);
                }
            }
            Ok(())
    }
    #[test]
    fn rollback_ok() -> Result<()>{
            let mut client = create_client()?;
            let transaction = super::TransactionUtil::start(&mut client , true)?;
            let result = transaction.rollback();
            match result {
                Ok(_) => assert!(true),
                Err(error) => {
                    println!("{:?}" , error.to_string());
                    assert!(false);
                }
            }
            Ok(())
    }
    
}