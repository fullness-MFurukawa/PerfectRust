//!
//! # ３章 変数と定数
//! サンプルプログラム
//! 

/// ### 3-1.変数と定数
/// #### リスト3-1 変数の宣言 
#[allow(dead_code)]
#[allow(unused_variables)]
pub fn declar_variables(){
    let x: i32 = 0; // 整数型変数を0で初期化して宣言
    let y = 100;    // 整数型変数を100で初期化して宣言(型推論)
}

/// ### 3-1.変数と定数
/// #### リスト3-2 ミュータブルな変数の宣言 
#[allow(dead_code)] 
pub fn declar_mutable_variables(){
    let mut x = 100; // 変更可能な変数を宣言
    x = x + 100;          // 変数に加算結果を代入して変更する
    println!("x = {}" , x);
}

/// ### 3-1.変数と定数
/// #### リスト3-3 シャドーイング
#[allow(dead_code)] 
pub fn shadowing(){
    // 変数の上書き
    let value1:i32 = 100;    // 変数value1(i32型)を宣言する
    println!("value1 = {}" , value1);
    let value1:i32 = 10;     // 変数が上書きされる 
    println!("value1 = {}" , value1);
    //　シャドーイング
    let value2:f32 = 100.1;  // 変数value2(f32型)を宣言する
    println!("value2 = {}" , value2);
    let value2:i32 = 100;    // 変数value2(i32型)を宣言する
    println!("value2 = {}" , value2);
}

