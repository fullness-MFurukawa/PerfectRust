//!
//!  12章 スレッドと非同期処理
//! 

mod basic;
mod cross_beam;
mod messaging_std;
mod messaging_crossbeam;
//mod exclusive_mutex;
//mod exclusive_rwlock;
//mod exclusive_shradelock;
mod asynchronous;
mod task_controller;

/// 12-8 async_stdクレート
/// リスト12-26 #[async_std:main]アトリビュート
/// 非同期実行に対応したmain()関数
#[async_std::main]
async fn main() {
    //basic::thread_controller_1(); // シンプルなスレッド   
    basic::thread_controller_2();   // シンプルなスレッド(Builder)
    //basic::thread_controller_3(); // メソッド
    //cross_beam::thread_controller_1();
    //cross_beam::thread_controller_2();
    //cross_beam::thread_controller_3();
    //cross_beam::thread_controller_4();
    //messaging_std::execute();
    //messaging_crossbeam::execute();
    //exclusive_mutex::use_mutex();
    //exclusive_rwlock::use_rwlock();
    //exclusive_shradelock::use_sharded_lock();
    /* 
    12-7 非同期実行
    リスト12-22 合計値を求める非同期関数　
    async{
        // 非同期関数の実行
        let total = calc_sum(vec![10,20,30,40,50]).await;
        println!("total = {}" ,total);
    };
    */
    //asynchronous::use_calc_sum();
    //asynchronous::use_calc_sum_2().await;
    //asynchronous::use_spawn().await;
    //asynchronous::use_builder().await;
    task_controller::customer_controller().await;
}

/* 
/// 12-7 非同期実行
/// リスト12-22 合計値を求める非同期関数　
async fn calc_sum(values:Vec<u64>) -> u64 {
    let mut total:u64 = 0;
    for value in values{
        total += value;
        std::thread::sleep(Duration::from_secs(1));
        println!("値1:{}" , value);
    }
    total
}
*/
