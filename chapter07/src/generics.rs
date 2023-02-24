//!
//!  7章 関数 サンプルプログラム
//! 

use std::ops::{Add , Sub};

/// ## 7-3.ジェネリクスとトレイト境界
/// ### リスト7-13 加算関数
#[allow(dead_code)]
fn add<T:Add<Output=T>>(x: T , y: T) -> T {
    x + y
}
/// ## 7-3.ジェネリクスとトレイト境界
/// ### リスト7-13 減算関数
#[allow(dead_code)]
fn sub<T:Sub<Output=T>>(x: T , y: T) -> T {
    x - y
}
/// ## 7-3.ジェネリクスとトレイト境界
/// ### リスト7-14 ジェネリク型を利用した関数の実行
#[allow(dead_code)]
pub fn use_add() {
    let r = add::<i32>(10 ,20);// i32型を利用する
    println!("use_add 10 + 20 = {}" , &r);
    let r = add::<f32>(10.05 , 20.06);// f32型を利用する
    println!("use_add 10.05 + 20.06 = {}" , &r);
}
/// ## 7-3.ジェネリクスとトレイト境界
/// ### リスト7-14 ジェネリク型を利用した関数の実行
#[allow(dead_code)]
pub fn use_sub() {
    let r = sub::<u64>(30 ,20);// u64型を利用する
    println!("use_sub 30 - 20 = {}" , &r);
    let r = sub::<f64>(100.005 , 20.006);// f64型を利用する
    println!("use_sub 100.005 - 20.006 = {}" , &r);
}
