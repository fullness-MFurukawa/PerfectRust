//!
//! 5章制御式 サンプルプログラム
//! 

/// ### 5-2.パターンマッチング
/// #### リスト5-4 match式
#[allow(dead_code)]
pub fn branch_1(){
    let x = 10;
    match  x { // 単純なmatch式
        1 => println!("値は1"),
        2 => println!("値は2"),
        _ => println!("値は不正です。")
    }
    match  x { // 複数の処理をするmacth式
        1 => {
            let y = 100;
            println!("y = {}" , y);
        },
        2 => {
            let y = 200;
            println!("y = {}" , y);
        },
        _ => {
            let y = 300;
            println!("y = {}" , y);
        }
    }    
}
/// ### 5-2.パターンマッチング
/// #### リスト5-5 match式
#[allow(dead_code)]
pub fn branch_2(){
    let x = "山田太郎";
    match x {
        "山田太郎" => println!("山田太郎です。"),
        "鈴木花子" => println!("鈴木花子です。"),
        _ => println!("誰?")
    }
}

/// ### 5-2.パターンマッチング
/// #### リスト5-6 match-let式
#[allow(dead_code)]
pub fn branch_3(){
    // 引数を10倍した結果を返すクロージャ
    let calc = |x:u32|{ x * 10 };
    let y = 3;
    let result = match y {
        1 => calc(10), // 引数10でクロージャを実行する
        2 => calc(20), // 引数20でクロージャを実行する
        3 => calc(30), // 引数30でクロージャを実行する
        _ => calc(0)   // 引数0でクロージャを実行する
    };
    println!("result = {}" , result);
}

/// ### 5-2.パターンマッチング
/// #### リスト5-7 RangeとOR演算子
#[allow(dead_code)]
pub fn branch_4(){
    let calc = |x:u32|{ x * 10 };
    let value = 30;
    let result = match value {
        1 ..= 3 => calc(10),        // 1～3の範囲
        4 ..= 6 => calc(20),        // 4～6の範囲
        7 ..= 9 => calc(30),        // 7～9の範囲
        10 | 20 | 30 => calc(40),   // 10または20または30
        21 ..= 25 | 31 ..= 35 => calc(50), // 21～25または31～35
        _ => calc(0)    // どのパターンにも一致しない
    };
    println!("result = {}" , result);
}    

/// ### 5-2.パターンマッチング
/// #### リスト5-8 ガードの利用
#[allow(dead_code)]
pub fn branch_5(){
    let value = (10 ,25);
    let result = match value {
        (x , y) if x == 0 && y == 0 =>          "xとyは0です。",
        (x , y) if x % 2 == 0 && y % 2== 0 =>   "xとyは偶数です。",
        (x , y) if x % 2 == 1 && y % 2== 1 =>   "xとyは奇数です。",
        _ => "どのパターンにも一致しません。"
    };
    println!("result = {}" , result);
}    