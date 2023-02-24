//!
//! 14章 PostgreSQL
//! 

use postgres::{Client, NoTls, Config };
use anyhow::Result;
use crate::section_1::params::ConnectParams;

/// ## 14-2.データベース接続
/// ### リスト14-3 データベース接続機能の実装
pub struct PostgresSampleClient; // データベース接続構造体
impl  PostgresSampleClient { // データベース接続構造体の実装
    /// ## Client構造体のconnect()関数を利用した接続
    /// ### リスト14-3 データベース接続機能の実装
    /// ### 引数    ConnectParam: 接続パラメータ構造体
    /// ### 戻り値  Client: 接続結果
    pub fn simple_connect(params: ConnectParams) -> Result<Client> {
        let client = Client::connect(params.connect_string().as_str(), NoTls)?;
        Ok(client)
    }
    /// ## Config構造体のconnect()メソッドを利用した接続
    /// ### リスト14-3 データベース接続機能の実装
    /// ### 引数    ConnectParam: 接続パラメータ構造体
    /// ### 戻り値  Client: 接続結果
    pub fn config_connect(params: ConnectParams) ->  Result<Client>{
        let client = Config::new()
            .host(params.get_host()) // 接続ホスト名
            .port(params.get_port().clone())// TCPポート番号
            .dbname(params.get_dbname())// データベース名
            .user(params.get_user())// ユーザー名
            .password(params.get_password())// パスワード
            .connect(NoTls)?;
        Ok(client)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// ## データベース接続のテスト
    /// ### リスト14-4 データベース接続機能の実装
    fn connect_ok() {
        // 接続情報の生成
        let params = ConnectParams::new(
            "localhost".to_owned() , 5432, "rust_sample".to_owned() ,
            "postgres".to_owned(), "postgres".to_owned());
        match super::PostgresSampleClient::simple_connect(params.clone()){
            Ok(client) => {
                println!("simple_connect:接続成功");
                let _ = client.close(); // 接続を閉じる
            }
            Err(error) => println!("{:?}" , error.to_string())
        };
        match super::PostgresSampleClient::config_connect(params.clone()){
            Ok(client) => {
                println!("config_connect:接続成功");
                let _ = client.close(); // 接続を閉じる
            }
            Err(error) => println!("{:?}" , error.to_string())
        };
    }    
}