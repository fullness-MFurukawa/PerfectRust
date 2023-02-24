/// 12章 スレッドと非同期処理
/// サンプルコード

use std::sync::{RwLock, Arc , mpsc};
use std::sync::mpsc::{Sender,Receiver};
use std::thread;
use std::thread::Builder;
use std::time::Duration;
use std::ops::Div;
use anyhow::Result;

#[derive(Debug)]
pub struct Calculator;
impl Calculator{
    /// ## 12-5 排他制御
    /// ### リスト12-18 計算対象のデータを生成する
    #[allow(dead_code)]
    fn create_data(sender:(Sender<()>,Sender<()>) , params: Arc<RwLock<Vec<u64>>>) -> Result<String> {
        {
            let mut vals = params.write().unwrap();
            println!("{:?}" , params);
            // 計算する値を格納する
            for num in 1..6{
                vals.push(num * 100);
            }
        }
        // 生成完了を通知
        sender.0.send(()).unwrap_or_else(|error|panic!("{:?}" ,error));
        sender.1.send(()).unwrap_or_else(|error|panic!("{:?}" ,error));
        Ok(String::from("計算値の生成完了"))
    }


    /// ## 12-5 排他制御
    /// ### リスト12-18  合計値を求める関数
    /// ### 引数 合計を計算する値
    /// ### 戻り値 合計値(u64)
    #[allow(dead_code)]
    fn calc_sum(receiver: Receiver<()> ,params:Arc<RwLock<Vec<u64>>>) -> Result<String> {
        // 値生成完了受信待ち
        receiver.recv().unwrap_or_else(|error|panic!("{:?}" ,error));
        let values = params.read().unwrap();
        let mut total:u64 = 0;
        for value in values.iter() { // 引数の合計値を求める
            total = total + value;
            std::thread::sleep(Duration::from_secs(2));
            println!("{} 値:{}" , thread::current().name().unwrap() , total);
        }
        Ok(total.to_string())
    }

    /// ## 12-5 排他制御
    /// ### リスト12-18  平均値を求める関数
    /// ### 引数 合計を計算する値
    /// ### 戻り値 合計値(u64)
    #[allow(dead_code)]
    fn calc_avg(receiver: Receiver<()> , params:Arc<RwLock<Vec<u64>>>) -> Result<String> {
        // 値生成完了受信待ち
        receiver.recv().unwrap_or_else(|error|panic!("{:?}" ,error));
        let values = params.read().unwrap();
        let mut avg:u64 = 0;
        for value in values.iter() { // 引数の合計値を求める
            avg = avg + value;
            std::thread::sleep(Duration::from_secs(2));
            println!("{} 値:{}" , thread::current().name().unwrap() , avg);
        }
        Ok(avg.div(values.iter().count() as u64).to_string())
    }
    /// ## 12-5 排他制御
    /// ### リスト12-19 3つの関数を実行する
    #[allow(dead_code)]
    pub fn use_rwlock() -> Result<()>{
        let (s_sender,s_receiver) = mpsc::channel::<()>();
        let (a_sender,a_receiver) = mpsc::channel::<()>();
        let params = Arc::new(RwLock::new(Vec::<u64>::new()));
        let mut handles = Vec::with_capacity(2);
        // 値を生成するスレッドを生成
        let builder = Builder::new().name(String::from("値生成スレッド")).stack_size(1024 * 3);
        let params_a = Arc::clone(&params); // Arcのクローンを生成
        handles.push(builder.spawn(move ||
            Self::create_data((s_sender , a_sender) , params_a).unwrap())?);
        // 合計を計算するスレッドを生成
        let builder = Builder::new().name(String::from("合計スレッド")).stack_size(1024 * 3);
        let params_b = Arc::clone(&params); // Arcのクローンを生成
        handles.push(builder.spawn(move ||
            Self::calc_sum(s_receiver , params_b).unwrap())?);
        // 平均を求めるスレッドを生成
        let builder = Builder::new().name(String::from("平均スレッド")).stack_size(1024 * 3);
        let params_c = Arc::clone(&params); // Arcのクローンを生成
        handles.push( builder.spawn(move ||
            Self::calc_avg(a_receiver , params_c).unwrap())?);
        // 実行結果を出力
        for handle in handles{
            let thread = handle.thread().clone();
            let result = handle.join()
                .unwrap_or_else(|error| panic!("{:?}", error));
            println!("{} 結果:{}" , thread.name().unwrap() , result);
        }
        Ok(())
    }
}
/// ## 12-5 排他制御
/// ### リスト12-19 3つの関数を実行する
#[allow(dead_code)]
pub fn use_rwlock(){
    let _ = Calculator::use_rwlock();
}
