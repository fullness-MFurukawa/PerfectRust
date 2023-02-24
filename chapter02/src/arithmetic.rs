//!
//! ## 算術演算子のサンプルコード
//! 

/// ### リスト2-1
/// #### 算術演算子の利用と結果の出力 
#[allow(dead_code)]
pub fn symbol() {
    let mut result = 10 + 20;   // 整数の加算
    println!("10 + 20 = {}" , result);
    result = 20 - 10;               // 整数の減算
    println!("20 - 10 = {}" , result);
    result = 20 * 10;               // 整数の乗算
    println!("20 * 10 = {}" , result);
    result = 10 / 5;                // 整数の除算
    println!("10 / 5 = {}" , result);
    result = 10 % 3;                // 整数の剰余
    println!("10 % 3 = {}" , result);
}
/// ### リスト2-3
/// #### 算術演算メソッドの利用と結果の出力 
/// #### 引数 x , y 計算対象の値
#[allow(dead_code)]
pub fn use_method(x: i32 , y: i32){
    // 算術演算トレイトの利用宣言をする
    use std::ops::{ Add , Sub , Mul , Div , Rem };
    // メソッドを使って算術演算を実行する
    println!("{} + {} = {}" , x , y , x.add(y));
    println!("{} - {} = {}" , x , y , x.sub(y));
    println!("{} * {} = {}" , x , y , x.mul(y));
    println!("{} / {} = {}" , x , y , x.div(y));
    println!("{} % {} = {}" , x , y , x.rem(y));
}
/// ### リスト2-4
/// #### オーバーフローを起こす計算
#[allow(dead_code)]
pub fn overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x + y;
    println!("{} + {} = {}" , x , y , result);
}
/// ### リスト2-5
/// #### オーバーフローを無視する
#[allow(dead_code)]
pub fn ignore_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x.wrapping_add(y);
    println!("{} + {} = {}" , x , y , result);
}

/// ### リスト2-6
/// #### Option<T>型でオーバーフローを確認する
#[allow(dead_code)]
pub fn check_option_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    match x.checked_add(y){
        Some(result) =>  println!("{} + {} = {}" , x , y , result),
        None => println!("オーバーフローしました")
    }
}
/// ### リスト2-7
/// #### 論理値でオーバーフローを確認する
#[allow(dead_code)]
pub fn check_boolean_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let (result , overflow) = x.overflowing_add(y);
    if overflow {
        println!("オーバーフローしました");
    }else{
        println!("{} + {} = {}" , x , y , result);
    }
}
/// ### リスト2-8
/// #### オーバーフローしたら最大値を返す
#[allow(dead_code)]
pub fn return_max_overflow() {
    let x: u8 = 100;
    let y: u8 = 200;
    let result = x.saturating_add(y);
    if result == u8::MAX {
        println!("オーバーフローしました");
    }else{
        println!("{} + {} = {}" , x , y , result);
    }
}