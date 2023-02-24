//!
//! 4章 基本データ型
//! サンプルプログラム
//! 

/// ### 4-4.文字型
/// #### リスト4-10 文字型リテラル
#[allow(dead_code)]
pub fn char_literal() {
    println!("a = {}" , 'a');
    println!("b = {}" , 'b');
    println!("あ = {}" , 'あ');
    println!("い = {}" , 'い');
}

/// ### 4-4.文字型
/// #### リスト4-11 文字型の定数
#[allow(dead_code)]
pub fn char_constant(){
    println!("MAX = {}" , char::MAX);
    println!("UNICODE_VERSION = {:?}" , char::UNICODE_VERSION);
}

/// ### 4-4.文字型
/// #### リスト4-12 文字型のメソッド
#[allow(dead_code)]
pub fn methods(){
    let x  = 'a';
    println!("is_alphabetic() = {}" , x.is_ascii_alphabetic());
    println!("is_numeric() = {}" , x.is_numeric());
    println!("is_lowercase() = {}" , x.is_lowercase());
    println!("is_uppercase() = {}" , x.is_uppercase());
    println!("to_ascii_lowercase() = {}" , x.to_ascii_lowercase());
    println!("to_ascii_uppercase() = {}" , x.to_ascii_uppercase());
}
