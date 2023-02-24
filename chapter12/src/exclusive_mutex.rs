/// 12章 スレッドと非同期処理
/// サンプルコード

use std::sync::Arc;
use std::time::Duration;
use std::sync::Mutex;
use std::thread;
use std::ops::Div;
use std::thread::Builder;
use anyhow::Result;
#[derive(Debug)]
pub struct Calculator;
impl Calculator {
    /// ## 12-5 排他制御
    /// ### リスト12-16 合計値を求める関数
    /// ### 引数 合計を計算する値
    /// ### 戻り値 合計値(u64)
    #[allow(dead_code)]
    fn calc_sum(params:Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut total:u64 = 0;
        {
            let values = params.lock() // 共有データをロックする
            .unwrap_or_else(|error| panic!("{:?}", error));
            println!("calc_sum ロック {:?}" , params );
            for value in values.iter() { // 引数の合計値を求める
                total = total + value;
                thread::sleep(Duration::from_secs(2));
                println!("{} 値:{}" ,
                thread::current().name().unwrap() , total);
            }
        }
        println!("summary ロック解除 {:?}" , params );
        total
    }
    /// ## 12-5 排他制御
    /// ### リスト12-16 平均値を求める関数
    /// # 引数 合計を計算する値
    /// # 戻り値 合計値(u64)
    #[allow(dead_code)]
    fn calc_avg(params:Arc<Mutex<Vec<u64>>>) -> u64 {
        let mut avg:u64 = 0;
        let mut count: u64 = 0;
        {
            let values = params.lock()// 共有データをロックする
                .unwrap_or_else(|error| panic!("{:?}", error));

            println!("calc_avg ロック {:?}" , params );
            count = values.iter().count() as u64;
            for value in values.iter() { // 引数の合計値を求める
                avg = avg + value;
                thread::sleep(Duration::from_secs(2));
                println!("{} 値:{}" ,
                thread::current().name().unwrap() , avg);
            }
        }
        println!("average ロック解除 {:?}" , params );
        avg.div(count)
    }
    /// ## 12-5 排他制御
    /// ### リスト12-17 2つの関数を実行する
    pub fn use_mutex() -> Result<()>{
        // 共有データを生成する
        let values:Vec<u64> = vec![ 10 , 20 , 30 , 40 , 50 ];
        let params = Arc::new(Mutex::new(values));

        let mut handles = Vec::with_capacity(2);
        // 合計を求めるスレッドを生成
        let builder = Builder::new().name(String::from("合計スレッド")).stack_size(1024 * 3);
        let _params = Arc::clone(&params); // クローンを生成
        handles.push(builder.spawn(move || Self::calc_sum(_params))?);
        // 平均を求めるスレッドを生成
        let builder = Builder::new().name(String::from("平均スレッド")).stack_size(1024 * 3);
        let _params = Arc::clone(&params); // クローンを生成
        handles.push( builder.spawn(move || Self::calc_avg(_params))?);

        // 実行結果を出力
        for handle in handles{
            let thread = handle.thread().clone();
            let result = handle.join().unwrap();
            println!("{} 結果:{}" , thread.name().unwrap() , result);
        }
        Ok(())
    }
}


/// ## 12-5 排他制御
/// ### リスト12-17 2つの関数を実行する
#[allow(dead_code)]
pub fn use_mutex(){
    let _ = Calculator::use_mutex();
}
