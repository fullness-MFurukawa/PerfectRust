//!
//! # ３章 変数と定数
//! サンプルプログラム
//! 

///  ### 3-1.変数と定数
///  #### リスト3-4 ソースファイル内でグローバルな文字列型定数
pub const SAMPLE_NAME: &str = "Rust サンプルプログラム";

///  ### 3-1.変数と定数
///  #### リスト3-4 定数の宣言と利用 
#[allow(dead_code)] 
pub fn use_constant() {
    const CALC_VALUE: i32 = 100;                // ローカルな整数型定数の宣言
    let result = 10 * CALC_VALUE;          // 定数を利用した演算
    println!("10 * CALC_VALUE = {}" , result);  // 結果の出力
    println!("SAMPLE_NAME = {}" , SAMPLE_NAME); // 文字列定数の利用
}