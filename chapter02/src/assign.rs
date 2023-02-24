//!
//! ## 代入演算子のサンプルコード
//! 

/// ### リスト2-9
/// #### 代入演算子の利用
#[allow(dead_code)]
pub fn assign_value(x: i64 , y: f64){
    let a = x;
    let b = y;
    println!("変数aの値 = {}" , a);
    println!("変数bの値 = {}" , b);
}

/// ### リスト2-10
/// #### 複合代入演算子の利用
#[allow(dead_code)]
pub fn compound_assign(mut x: i32 , y: i32){
    x += y;
    println!("x += y = {}" , x);
    x -= y;
    println!("x -= y = {}" , x);
    x *= y;
    println!("x *= y = {}" , x);
    x /= y;
    println!("x /= y = {}" , x);
    x %= y;
    println!("x %= y = {}" , x);
}

/// ### リスト2-11
/// #### 複合代入演算メソッドの利用
#[allow(dead_code)]
pub fn compound_assign_method(mut x: i32 , y: i32){
    use std::ops::{AddAssign , SubAssign , MulAssign , DivAssign , RemAssign};
    x.add_assign(y);
    println!("x += y = {}" , x);
    x.sub_assign(y);
    println!("x -= y = {}" , x);
    x.mul_assign(y);
    println!("x *= y = {}" , x);
    x.div_assign(y);
    println!("x /= y = {}" , x);
    x.rem_assign(y);
    println!("x %= y = {}" , x);
}