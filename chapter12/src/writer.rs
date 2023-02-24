/// 12章 スレッドと非同期処理
/// サンプルコード

use std::fs::File;
use std::env;
use csv::WriterBuilder;
use async_std::channel::Receiver;
use anyhow::Result;
use crate::customer::Customer;
pub struct SampleWriter;
impl SampleWriter {
    /// ## 12-9 タスク間通信
    /// ### リスト12-30 受信した顧客情報をCSVファイルに格納する
    /// ### 引数: receiver 顧客情報受信用レシーバ
    /// #### 戻り値: 終了メッセージ
    pub async fn csv_writer(&self, receiver: Receiver<Customer>) -> Result<String> {
        // 受信データを一時保存するVecを生成
        let mut customers:Vec<Customer> = Vec::new();
        loop {
            let customer = receiver.recv().await?; // 顧客情報受信待ち
            if customer.get_name() == "end" { // 氏名が"end"なら終了
                break;
            }
            customers.push(customer); // 受信した顧客データを一時保存
        }
        // ファイルパスを取得
        let path = env::current_dir() .map(|path| path.join("resources\\customer.csv") )?;
        // Writerを生成
        let mut writer = WriterBuilder::new().from_path(path)?;
        for customer in customers {  // 受信した顧客情報をシリアライズ
            writer.serialize(customer)?;
        }
        Ok(String::from("csv_writer終了"))
    }
    /// ## 12-9 タスク間通信
    /// ### リスト12-31 受信した顧客情報をJSONファイルに格納する
    /// ### 引数: receiver 顧客情報受信用レシーバ
    /// ### 戻り値: 終了メッセージ
    pub async fn json_writer(&self, receiver: Receiver<Customer>) -> Result<String> {
        let mut customers:Vec<Customer> = Vec::new();
        loop {
            let customer = receiver.recv().await?; // 顧客情報受信待ち
            if customer.get_name() == "end" { // 氏名が"end"なら終了
                break;
            }
            customers.push(customer);
        }
        let path = env::current_dir()// ファイルパスを取得
            .map(|path| path.join("resources\\customer.json"))?;
        // Writerを生成
        let writer = File::create(path)
            .map(|file| std::io::BufWriter::new(file))?;
        // 受信した顧客情報をシリアライズ
        serde_json::to_writer(writer , &customers)?;
        Ok(String::from("json_writer終了"))
    }

}
