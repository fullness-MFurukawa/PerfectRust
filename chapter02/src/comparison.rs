//!
//! ## 比較演算子のサンプルコード
//! 


/// ### リスト2-12
/// #### 比較演算子の利用
#[allow(dead_code)]
pub fn symbol(x: i32 , y: i32){
    println!("{} == {} = {}" , x , y , x == y);
    println!("{} != {} = {}" , x , y , x != y);
    println!("{} <= {} = {}" , x , y , x <= y);
    println!("{} >= {} = {}" , x , y , x >= y);
    println!("{} < {}  = {}" , x , y , x < y);
    println!("{} > {}  = {}" , x , y , x > y);
}

/// ### リスト2-13
/// #### 比較演算メソッドの利用
#[allow(dead_code)]
pub fn methods(x: i32 , y: i32){
    println!("{} eq {} = {}" , x , y , x.eq(&y));
    println!("{} ne {} = {}" , x , y , x.ne(&y));
    println!("{} le {} = {}" , x , y , x.le(&y));
    println!("{} ge {} = {}" , x , y , x.ge(&y));
    println!("{} lt {} = {}" , x , y , x.lt(&y));
    println!("{} gt {} = {}" , x , y , x.gt(&y));
}