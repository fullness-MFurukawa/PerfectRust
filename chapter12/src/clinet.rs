/// 12章 スレッドと非同期処理
/// サンプルコード

use anyhow::Result;
use async_std::channel::Sender;
use crate::customer::Customer;
use crate::station::enter_station;
pub struct Client;
impl Client {
    /// ## 12-9 タスク間通信
    /// ### リスト12-32 入力された顧客情報をWriterに送信する
    /// ### 引数: receiver 顧客情報送信用センダー
    /// ### 戻り値: 終了メッセージ
    pub async fn entry(&self , sender:(Sender<Customer>,Sender<Customer>)) -> Result<String> {
        loop{
            let name = enter_station("氏名を入力してください");
            let email = enter_station("メールアドレスを入力してください");
            // Customer構造体を生成
            let customer = Customer::new(name.clone(),email.clone());
            // Writerにデータを送信
            sender.0.send(customer.clone()).await?;
            sender.1.send(customer).await?;
            if name == "end" { // nameがendなら終了
                break;
            }
        }
        Ok(String::from("Client終了"))
    }
}
