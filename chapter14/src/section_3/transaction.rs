//!
//! 14章 PostgreSQL
//!
use anyhow::Result;
use tokio_postgres::{Client, Transaction};
/// ## 14-7.非同期実行
/// ### リスト14-22 トランザクション制御機能の実装
pub struct AsyncTransactionUtil;
impl AsyncTransactionUtil {
    /// ## トランザクションを開始する
    /// ### 引数 client: データベース接続
    /// ### 引数 mode true:読取専用トランザクション
    /// ### 戻り値 Transaction:開始したトランザクション
    pub async fn start<'a>(client: &'a mut Client, mode: bool) -> Result<Transaction<'a>>{
        let transaction =
            client.build_transaction().read_only(mode).start().await?;
        Ok(transaction)
    }   
}
#[cfg(test)]
pub mod tests{
    use super::*;
    use anyhow::Result;
    use crate::section_3::connect::AsyncSimpleClient;
    #[tokio::test]
    async fn start_transaction() -> Result<()> {
        let mut client = AsyncSimpleClient::func_connect().await?;
        let _ = AsyncTransactionUtil::start(&mut client , true).await?;
        //assert!(true);

        let mut client = AsyncSimpleClient::func_connect().await?;
        let _ = AsyncTransactionUtil::start(&mut client , false).await?;
        //assert!(true);
        Ok(())
    }

}