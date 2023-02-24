///
/// 8章 構造体のサンプルコード
/// 


/// ## 8-2.タプル型とユニット型
/// ### リスト8-4 座標を表すタプル型構造体
#[allow(dead_code)]
struct Coordinates(usize , usize);
#[allow(dead_code)]
pub fn generate_tuple(){
     let c = Coordinates(100 , 200); // 座標を表すタプル型構造体の生成
     println!("{}" , c.0);
     println!("{}" , c.1); 
}

use std::borrow::Borrow;
/// ## 8-2.タプル型とユニット型
/// ### リスト8-5 ユニット型構造体
#[allow(dead_code)]
struct OneState;
#[allow(dead_code)]
pub fn generate_unit(){
    let o = OneState; // ユニット型構造体の生成
    println!("{:p}" , o.borrow());
}