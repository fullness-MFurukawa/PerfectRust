//!
//! 4章 基本データ型
//! サンプルプログラム
//! 

/// ### 4-1.整数型
/// #### リスト4-1 整数リテラル
#[allow(dead_code)]
pub fn integer_literal() {
    println!("a = {}" , 100i64);    // 10進数形式の整数リテラル
    println!("b = {}" , 0o123u64);  // 8進数形式の整数リテラル 
    println!("c = {}" , 0xf124i32); // 16進数形式の整数リテラル
    println!("d = {}" , 0o123_u64); // 8進数形式の整数リテラル 
    println!("e = {}" , 0x_f124i32);// 16進数形式の整数リテラル
    println!("f = {}" , 123_456_789);// 3桁単位でアンダーバーで連結
}
/// ### 4-1.整数型
/// #### リスト4-2 バイトリテラル
#[allow(dead_code)]
pub fn byte_literal(){
    println!("a = {}" , b'\x01');
    println!("b = {}" , b'\x0A');
    println!("c = {}" , b'\x1D');
}
/// ### 4-1.整数型
/// #### リスト4-3 i32型の定数
#[allow(dead_code)]
pub fn i32_constant(){
    println!("BITS  = {}" , i32::BITS);
    println!("MIN   = {}" , i32::MIN);
    println!("MAX   = {}" , i32::MAX);
}
/// ### 4-1.整数型
/// #### リスト4-4 整数型の主なメソッド
#[allow(dead_code)]
pub fn methods(){
    println!("abs() = {}" , -10i32.abs()); // 絶対値を求める
    println!("signum() = {}" , -10i32.signum());// 符号を表す値を返す
    println!("pow() = {}" , 10i32.pow(3)); // 累乗結果を返す(10の3乗を求める)
    println!("is_negative() = {}" , 10i32.is_negative()); // 負の値を確認する
    println!("is_positive() = {}" , 10i32.is_positive()); // 正の値を確認する
    // 値をコピーする
    let x = 1000;
    let y = x.clone();
    println!("clone() = {} " , y);
    println!("to_string() = {}" , y.to_string()); // 文字列に変換する
}