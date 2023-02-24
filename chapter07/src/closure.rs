//!
//!  7章 関数 サンプルプログラム
//!


/// ## 7-10.クロージャ
/// ### リスト7-51 基本的なクロージャ
#[allow(dead_code)]
pub fn closure_sum() {
    // 合計を求めるクロージャ
    let sum = |values: &Vec<i32>| {
        let mut sum = 0;
        for value in values.iter(){
            sum += value;
        }
        sum
    };
    let values = vec![1 , 2 , 3 , 4 , 5];
    println!("sum = {}" , sum(&values));
}

/// ## 7-10.クロージャ
/// ### リスト7-52 moveキーワード
#[allow(dead_code)]
pub fn move_1() {
    let values = vec![1 , 2 , 3 , 4 , 5];
    // 合計を求めるクロージャ
    let sum = || {
        let mut sum = 0;
        for value in values.iter(){
            sum += value;
        }
        sum
    };
    println!("sum = {}" , sum());
    println!("values = {:?}" , values);
}
/* 
/// ## 7-10.クロージャ
/// ### リスト7-53 moveキーワード
#[allow(dead_code)]
pub fn move_2() {
    let values = vec![1 , 2 , 3 , 4 , 5];
    // 合計を求めるクロージャ
    let sum = move || {
        let mut sum = 0;
        for value in values.iter(){
            sum += value;
        }
        sum
    };
    println!("sum = {}" , sum());
    println!("values = {:?}" , values);
}
*/

use std::ops::Fn;
/// ## 7-10.クロージャ
/// ### リスト7-55 クロージャを返す関数
#[allow(dead_code)]
pub fn impl_1(values: Vec<i32>) -> impl Fn() -> i32 {
    move || { // 合計を求めるクロージャ
        let mut sum = 0;
        for value in values.iter(){
            sum += value;
        }
        sum
    }
}
/// ## 7-10.クロージャ
/// ### リスト7-56 クロージャを受け取る関数
#[allow(dead_code)]
pub fn where_1<F>(f: F) where F:Fn() -> i32 {
    let sum = f();
    println!("sum = {}" , sum);
}
/// ## 7-10.クロージャ
/// ### リスト7-57 関数を実行する
#[allow(dead_code)]
pub fn use_impl_where_1(){
    let values = vec![1 , 2 , 3 , 4 , 5];
    let f = impl_1(values);  // クロージャを返す関数
    where_1(f); // クロージャを実行する関数
}


/// ## 7-10.クロージャ
/// ### リスト7-58 引数付きクロージャ
#[allow(dead_code)]
pub fn impl_2() -> impl Fn(Vec<i32>) -> i32 {
    move |values: Vec<i32>| { // 合計を求めるクロージャ
        let mut sum = 0;
        for value in values.iter(){
            sum += value;
        }
        sum
    }
}
/// ## 7-10.クロージャ
/// ### リスト7-58 引数付きクロージャ
#[allow(dead_code)]
pub fn where_2<F>(f: F) where F:Fn(Vec<i32>) -> i32 {
    let values = vec![1 , 2 , 3 , 4 , 5];
    let sum = f(values);
    println!("sum = {}" , sum);
}
/// ## 7-10.クロージャ
/// ### リスト7-58 引数付きクロージャ
#[allow(dead_code)]
pub fn use_impl_where_2(){
    let f = impl_2();  // クロージャを返す関数
    where_2(f); // クロージャを実行する関数
}