/// 12章 スレッドと非同期処理
/// サンプルコード

use crossbeam::channel::{Sender , Receiver};
use chapter12::station::{Station,enter_station};
use chapter12::stations::Stations;

#[allow(dead_code)]
#[derive(Default)]
pub struct Client;
impl Client {
    /// ## 12-4 スレッド間通信
    /// ### リスト12-13 Providerに検索要求を出し結果を受け取るメソッド
    /// ### 引数 Providerへの送信チャネル p_sender: Sender<String>
    /// ### 引数 Clinetの受信チャネル c_receiver: Receiver<String>
    #[allow(dead_code)]
    pub fn search_request(&self,p_sender: Sender<String>,c_receiver: Receiver<String>) {
        loop {
            let entry_name = enter_station("駅名を入力してください");
            // 入力された駅名をProviderに送信
            p_sender.send(entry_name.clone())
                .unwrap_or_else(|error| println!("{:?}", error));
            if entry_name == "end" { // endなら終了
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
#[derive(Default)]
pub struct Provider;
impl Provider {
    /// ## 12-4 スレッド間通信
    /// ### リスト12-14 受信した駅名で検索した結果を送信するメソッド
    /// # 引数 Clientへの送信チャネル c_sender: Sender<String>
    /// # 引数 Providerの受信チャネル p_receiver: Receiver<String>
    #[allow(dead_code)]
    pub fn search_service(&self,c_sender: Sender<String>, 
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

use crossbeam::channel::bounded;
use crossbeam::thread;
/// ## 12-4 スレッド間通信
/// ### リスト12-15 通信の確認
#[allow(dead_code)]
pub fn execute() {
    // Providerのチャネル生成(5件のメッセージまで保持)
    let (p_sender,p_receiver) = bounded::<String>(5);
    // Clientのチャネル生成(5件のメッセージまで保持)
    let (c_sender,c_receiver) = bounded::<String>(5);
    thread::scope(|scope| {
        // Providerスレッド生成
        let p_handle = scope.spawn(|_| {
            let provider = Provider::default();
            provider.search_service(c_sender, p_receiver);
        });
        // Clientスレッド生成
        let c_handle = scope.spawn(|_| {
            let client = Client::default();
            client.search_request(p_sender , c_receiver);
        });
        // 終了待ち
        p_handle.join().unwrap_or_else(|error|panic!("{:?}" , error));
        c_handle.join().unwrap_or_else(|error|panic!("{:?}" , error));
    }).unwrap();
}