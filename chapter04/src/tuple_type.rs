//!
//! 4章 基本データ型
//! サンプルプログラム
//! 

/// ### 4-6.タプル型
/// #### リスト4-17 タプル型の宣言
#[allow(dead_code)]
pub fn declare(){
    // タプル型変数を宣言する
    let x:(i32 , &str , bool) = (100 , "Hello" , true);
    // 変数a,b,cで値を受け取る
    let (a , b , c) = (100 , "Hello" , true);
    // 変数l,mで値を受け取る
    let (l , _ , m) = x;
    println!("x = {:?}" , x);
    println!("a = {:?} , b = {:?} , c = {:?}" , a , b , c);
    println!("l = {:?} , m = {:?}" , l , m);
}

/// ### 4-6.タプル型
/// #### リスト4-18 インデックスの利用
#[allow(dead_code)]
pub fn calc(value: (i32 , i32)){
    println!("{} + {} = {}" , value.0 , value.1 , value.0 + value.1);
    println!("{} - {} = {}" , value.0 , value.1 , value.0 - value.1);
    println!("{} * {} = {}" , value.0 , value.1 , value.0 * value.1);
    println!("{} / {} = {}" , value.0 , value.1 , value.0 / value.1);
    println!("{} % {} = {}" , value.0 , value.1 , value.0 % value.1);
}

/// ### 4-6.タプル型
/// #### リスト4-19 メソッド
#[allow(dead_code)]
pub fn methods(){
    let a:(i32 , i32 , i32) = (100 , 200 , 300);
    println!("clone() = {:?}" , a.clone()); // 値の複製を取得する
    println!("eq() = {}" ,  a.eq(&(100 , 200 , 300))); // 値を比較する
    println!("cmp() = {:?}" , a.cmp(&(200 , 200 , 300))); // 値を比較する
    println!("max() = {:?}" , a.max((100 , 200 , 400)));  // 比較結果を返す
    println!("min() = {:?}" , a.min((10 , 20 , 30))); // 比較結果を返す
}

