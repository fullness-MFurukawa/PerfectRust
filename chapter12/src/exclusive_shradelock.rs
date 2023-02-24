/// 12章 スレッドと非同期処理
/// サンプルコード

use std::time::Duration;
use std::sync::Arc;
use std::ops::Div;
use crossbeam::thread;
use crossbeam::sync::ShardedLock;
use crossbeam::channel::{bounded , Sender ,Receiver};
#[derive(Debug , Default)]
pub struct Calculator;
impl Calculator{
    /// ## 12-6 crossbeamクレートの排他制御
    /// ### リスト12-19 計算対象のデータを生成する
    fn create_data(&self,sender:(Sender<()>,Sender<()>) , params: Arc<ShardedLock<Vec<u64>>>) -> String {
        {
            let mut vals = params.write()
                .unwrap_or_else(|error| panic!("{:?}" , error));
            println!("{:?}" , params);
            // 計算する値を格納する
            for num in 1..6{
                vals.push(num * 100);
            }
        }
        // 生成完了を通知
        sender.0.send(()).unwrap_or_else(|error|panic!("{:?}" ,error));
        sender.1.send(()).unwrap_or_else(|error|panic!("{:?}" ,error));
        String::from("計算値の生成完了")
    }

    /// ## 12-6 crossbeamクレートの排他制御
    /// ### リスト12-19 合計値を求める関数
    /// ### 引数 合計を計算する値
    /// ### 戻り値 合計値(u64)
    fn calc_sum(&self,receiver: Receiver<()> ,params:Arc<ShardedLock<Vec<u64>>>) -> String {
        // 値生成完了受信待ち
        receiver.recv().unwrap_or_else(|error|panic!("{:?}" ,error));
        let values = params.read()
            .unwrap_or_else(|error| panic!("{:?}", error));
        let mut total:u64 = 0;
        for value in values.iter() { // 引数の合計値を求める
            total = total + value;
            std::thread::sleep(Duration::from_secs(2));
            println!("合計スレッド 値:{}" ,  total);
        }
        total.to_string()
    }
    /// ## 12-6 crossbeamクレートの排他制御
    /// ### リスト12-19 平均値を求める関数
    /// ### 引数 合計を計算する値
    /// ### 戻り値 合計値(u64)
    fn calc_avg(&self,receiver: Receiver<()> , params:Arc<ShardedLock<Vec<u64>>>) -> String {
        // 値生成完了受信待ち
        receiver.recv().unwrap_or_else(|error|panic!("{:?}" ,error));
        let values = params.read()
            .unwrap_or_else(|error| panic!("{:?}", error));
        let mut avg:u64 = 0;
        for value in values.iter() { // 引数の合計値を求める
            avg = avg + value;
            std::thread::sleep(Duration::from_secs(2));
            println!("平均スレッド 値:{}" , avg);
        }
        avg.div(values.iter().count() as u64).to_string()
    }
    /// ## 12-6 crossbeamクレートの排他制御
    /// ### リスト12-21 3つの関数を実行する
    pub fn use_sharded_lock(&self) {
        thread::scope(|scope| {
            // 合計計算機能のチャネル生成
            let (s_sender, s_receiver) = bounded::<()>(5);
            // 平均計算機能のチャネル生成
            let (a_sender, a_receiver) = bounded::<()>(5);
            // 共有する値を生成
            let params = Arc::new(ShardedLock::new(Vec::<u64>::new()));
            let mut handles = Vec::with_capacity(3);
            // 値を生成するスレッド
            let params_a = Arc::clone(&params);
            handles.push(scope.builder()
                .name(String::from("値生成スレッド")).stack_size(1024 * 3)
                .spawn(|_| self.create_data((s_sender, a_sender), params_a))
                .unwrap_or_else(|error| panic!("{:?}", error)));
            // 合計を計算するスレッド
            let params_b = Arc::clone(&params);
            handles.push(scope.builder()
                .name(String::from("合計スレッド")).stack_size(1024 * 3)
                .spawn(|_| self.calc_sum(s_receiver , params_b))
                .unwrap_or_else(|error| panic!("{:?}", error)));
            // 平均を計算するスレッド
            let params_c = Arc::clone(&params);
            handles.push(scope.builder()
                .name(String::from("平均スレッド")).stack_size(1024 * 3)
                .spawn(|_| self.calc_avg(a_receiver , params_c))
                .unwrap_or_else(|error| panic!("{:?}", error)));
            // スレッド実行結果を出力
            for handle in handles{
                let thread = handle.thread().clone();
                let result = handle.join().unwrap_or_else(|error| panic!("{:?}",error));
                println!("{} {}" , thread.name().unwrap() , result);
            }
        }).unwrap();
    }
}
/// ## 12-6 crossbeamクレートの排他制御
/// ### リスト12-21 3つの関数を実行する
#[allow(dead_code)]
pub fn use_sharded_lock(){
    let c = Calculator::default();
    c.use_sharded_lock();
}

