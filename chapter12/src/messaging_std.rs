/// 12章 スレッドと非同期処理
/// サンプルコード


use std::sync::mpsc::{Sender, Receiver};
use chapter12::station::{Station,enter_station};
use chapter12::stations::Stations;
#[allow(dead_code)]
pub struct Client;
impl Client {
    /// ## 12-4 スレッド間通信
    /// ### リスト12-10 Providerに検索要求を出し結果を受け取るメソッド
    /// ### 引数 Providerへの送信チャネル p_sender: Sender<String>
    /// ### 引数 Clinetの受信チャネル c_receiver: Receiver<String>
    #[allow(dead_code)]
    pub fn search_request(p_sender: Sender<String>,c_receiver: Receiver<String>) {
        loop {
            let entry_name = enter_station("駅名を入力してください");
            // 入力された駅名をProviderに送信
            p_sender.send(entry_name.clone())
                .unwrap_or_else(|error| println!("{:?}", error));
            // endなら終了
            if entry_name == "end" {
                break;
            }
            // Providerからの検索結果受信
            c_receiver.recv().and_then(|result| {
                    println!("{:?}", result);
                    Ok(())
                }).unwrap_or_else(|error| println!("{:?}", error));
        }
        println!("Client 終了");
    }
}

#[allow(dead_code)]
pub struct Provider;
impl Provider { 
    /// ## 12-4 スレッド間通信
    /// ### リスト12-11 受信した駅名で検索した結果を送信するメソッド
    /// ### 引数 Clientへの送信チャネル c_sender: Sender<String>
    /// ### 引数 Providerの受信チャネル p_receiver: Receiver<String>
    #[allow(dead_code)]
    pub fn search_service(c_sender: Sender<String>, 
    p_receiver: Receiver<String>) {
        let stations = Stations::<Station>::new();
        loop {
            // 入力された駅名の受信
            let entry_name = p_receiver.recv()
                .unwrap_or_else(|error| panic!("{:?}", error));
            if entry_name == "end" { // endなら終了
                break;
            }
            // 駅名を検索
            let result = match stations.search_by_name(entry_name){
                Some(station) => station.get_message(),
                None => String::from("該当する駅がみつかりません")
            };
            c_sender.send(result) // 検索結果をClientに送信
                .unwrap_or_else(|error| panic!("{:?}" , error));
        }
        println!("Provider 終了");
    }
}

use std::thread;
use std::sync::mpsc;
/// ## 12-4 スレッド間通信
/// ### リスト12-12 通信の確認
#[allow(dead_code)]
pub fn execute() {
    // チャネルを生成する
    let (c_sender,c_receiver) = mpsc::channel::<String>();
    let (p_sender,p_receiver) = mpsc::channel::<String>();
    // Providerスレッドを生成する
    let p_handle = thread::spawn(move ||
        Provider::search_service(c_sender , p_receiver));
    // Clientスレッドを生成する
    let c_handle = thread::spawn(move ||
        Client::search_request(p_sender,c_receiver));
    // 終了待ち
    p_handle.join().unwrap_or_else(|error| panic!("{:?}", error));
    c_handle.join().unwrap_or_else(|error| panic!("{:?}", error));
}

