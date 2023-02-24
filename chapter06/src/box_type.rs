//!
//! 6章 ライブラリ型 サンプルプログラム
//! 

/// 6-3.Box<T>
/// リスト6-16 インスタンスの生成
#[allow(dead_code)]
pub fn instantiate(){
    use std::ops::Add;
    // i32型整数をボックス化する
    let x = Box::new(100); 
    let y = Box::new(200); 
    println!("x + y = {}" , x.add(*y));// ボックス化された値のメソッドを利用する
    println!("x + y = {}" , *x + *y);  // ボックス化された値を取り出して利用する
    // &str型とString型をボックス化する
    let a = Box::new("ABCXYZ");
    let b = Box::new(String::from("XYZ"));
    println!("contains() = {}" , &a.contains(&b.as_str()));// ボックス化された値のメソッドを利用する
}
