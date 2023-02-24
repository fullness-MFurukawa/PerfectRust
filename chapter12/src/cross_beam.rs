/// 12章 スレッドと非同期処理
/// サンプルコード

use std::time::Duration;
use std::sync::Barrier;
use std::sync::Arc;
use std::thread::Builder;
use crossbeam::thread;
use crossbeam::sync::WaitGroup;

#[derive(Debug , Default)]
pub struct Summary;
impl Summary {
    /// ## 合計値を求めるメソッド
    /// ### 引数 合計を計算する値
    /// ### 戻り値 合計値(u64)
    #[allow(dead_code)]
    fn summary(&self,name: &str,values:  Vec<u64>) -> u64 {
        let mut total:u64 = 0;
        for value in values { // 引数の合計値を求める
            total = total + value;
            std::thread::sleep(Duration::from_secs(2));
            println!("{} 値:{}" , name, total);
        }
        total
    }
    /// ## 12-2.グリーンスレッド
    /// ### リスト12-6 合計値を求めるスレッドを実行するメソッド
    /// ### 引数 処理の名前
    /// ### 引数 合計を計算する値
    /// ### 戻り値 スレッドのハンドラ
    #[allow(dead_code)]
    pub fn summary_thread(&self) {
        // スコープを設定
        thread::scope(|scope| {
            //スコープ付きスレッドの生成と実行
            let handle1 = scope.spawn(|_| self.summary("summary1",vec![10,20,30,40,50]));
            let handle2 = scope.spawn(|_| self.summary("summary2",vec![100,200,300,400,500]));
            // スレッドの終了待ち
            let total1 = handle1.join().unwrap_or_else(|error| panic!("{:?}" , error));
            let total2 = handle2.join().unwrap_or_else(|error| panic!("{:?}" , error));
            // 終了結果を出力する
            println!("total1の合計値:{}" , total1);
            println!("total2の合計値:{}" , total2);
        }).unwrap();
    }
    /// ## 12-2.グリーンスレッド
    /// ### リスト12-7 ScopedThreadBuilder構造体の利用
    #[allow(dead_code)]
    pub fn use_builder(&self) {
        thread::scope(|scope| {
            let handle1 = scope.builder()
                .name(String::from("sum1"))//スレッドの名称を設定する
                .stack_size(1024 * 3)//スタックサイズを設定する
                .spawn(|_| {
                    self.summary(std::thread::current().name().unwrap(),
                    vec![10, 20, 30, 40, 50])
            }).unwrap_or_else(|error| panic!("{:?}", error));
            // スレッドの終了待ち
            let total1 = handle1.join()
                .unwrap_or_else(|error| panic!("{:?}" , error));
            // 終了結果を出力する
            println!("total1の合計値:{}" , total1);
        }).unwrap();
    }

    #[allow(dead_code)]
    fn summary_a(values:  Vec<u64>) -> u64 {
        let mut total:u64 = 0;
        for value in values { // 引数の合計値を求める
            total = total + value;
            std::thread::sleep(Duration::from_secs(2));
        }
        total
    }
    /// ## 12-2.グリーンスレッド
    /// ### リスト12-8  Barrierを利用した終了の同期化
    #[allow(dead_code)]
    pub fn use_barrier() {
        // スレッドのハンドル(JoinHandle)を格納するVec
        let mut handles = Vec::with_capacity(3);
        // ArcでラップしたBarrierを生成する
        let barrier = Arc::new(Barrier::new(3));
        let mut num: u64 = 0;
        while 2 >= num {
            let arc = Arc::clone(&barrier); // Arcのクローンを生成する
            handles.push(// スレッドをVecに登録する
                Builder::new()
                .name(format!("{}{}","summary",num)).stack_size(1024 * 5)
                .spawn(move || {
                    let data: Vec<u64> = 
                    vec![10+num, 20+num, 30+num, 40+num , 50+num];
                    let result = Self::summary_a(data);
                    let wresult = arc.wait(); // wait()メソッドで終了を待つ
                    println!("{} 終了:{}" , std::thread::current().name().unwrap() , wresult.is_leader());
                    result
                }).unwrap_or_else(|error| panic!("{:?}",error)));
            num += 1;
        }
        // スレッドの実行結果を出力する
        for handle in handles {
            let thread = handle.thread().clone();
            let result = handle.join()
            .unwrap_or_else(|error| panic!("{:?}",error));
            println!("{} 合計:{}" , thread.name().unwrap() , result);
        }
    }

    /// ## 12-3.スレッド終了の同期化
    /// ### リスト12-9WaitGroupを利用した終了の同期化
    #[allow(dead_code)]
    pub fn use_wait_group(&self) {
        thread::scope(|scope| {
            let mut handles = Vec::with_capacity(3);
            let wait_group = WaitGroup::new(); // WaitGroupの生成
            let mut num: u64 = 0;
            while 2 >= num {
                let wg = wait_group.clone(); // クローンを生成
                handles.push(scope.builder()
                .name(format!("{}{}" , "summary" , num))
                .stack_size(1024 * 3)
                .spawn(move |_| {
                    let result = self.summary(
                    std::thread::current().name().unwrap(),
                    vec![10+num, 20+num , 30+num , 40+num , 50+num]);
                    drop(wg); // WaitGroupを破棄
                    result })
                    .unwrap_or_else(|error| panic!("{:?}", error)));
                num += 1;
            }
            wait_group.wait(); // すべてのWaitGroupが無くなるまで待機
            for handle in handles{
                let thread = handle.thread().clone();
                let result = handle.join()
                .unwrap_or_else(|error| panic!("{:?}", error));
                println!("{} 合計:{}" , thread.name().unwrap() , result);
            }
        }).unwrap();
    }

}





/// ## 12-2.グリーンスレッド
/// ### リスト12-6 合計値を求めるスレッドを実行する関数
#[allow(dead_code)]
pub fn thread_controller_1() {
    let s = Summary::default();
    s.summary_thread();
}
/// ## 12-2.グリーンスレッド
/// ### リスト12-7 ScopedThreadBuilder構造体の利用
#[allow(dead_code)]
pub fn thread_controller_2() {
    let s = Summary::default();
    s.use_builder();
}
/// ## 12-2.グリーンスレッド
/// ### リスト12-7 ScopedThreadBuilder構造体の利用
#[allow(dead_code)]
pub fn thread_controller_3() {
    Summary::use_barrier();
}
/// ## 12-2.グリーンスレッド
/// ### リスト12-7 ScopedThreadBuilder構造体の利用
#[allow(dead_code)]
pub fn thread_controller_4() {
    let s = Summary::default();
    s.use_wait_group();
}

