/// 12章 スレッドと非同期処理
/// サンプルコード


use anyhow::Result;
use async_std::channel::unbounded;
use async_std::task;
use async_std::task::JoinHandle;
use chapter12::customer::Customer;
use chapter12::writer::SampleWriter;
use chapter12::clinet::Client;

/// ## 12-9 タスク間通信
/// ### リスト12-33 ３つのタスクの生成と実行制御
#[allow(dead_code)]
pub async fn customer_controller(){
    // Writer用チャネルの生成
    let (csv_sender, csv_receiver) = unbounded::<Customer>();
    let (json_sender, json_receiver) = unbounded::<Customer>();
    // タスクハンドルの保存
    let mut handles:Vec<JoinHandle<Result<String>>> = Vec::new();
    // CSVファイル用Writerの生成
    handles.push(task::spawn(async {
        let writer = SampleWriter;
        writer.csv_writer(csv_receiver).await }));
    // JSONファイル用Writerの生成
    handles.push(task::spawn(async move{
        let writer = SampleWriter;
        writer.json_writer(json_receiver).await }));
    // Clientの生成
    handles.push(task::spawn( async move{
        let client = Client;
        client.entry((csv_sender,json_sender)).await }));
    // 終了待ち
    for handle in handles {
        match handle.await {
            Ok(result) => println!("{}" , result),
            Err(error) => println!("{:?}" ,error)
        }
    }
}