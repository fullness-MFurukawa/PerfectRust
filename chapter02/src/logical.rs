//!
//! ## 論理演算子のサンプルコード
//! 

/// ### リスト2-14
/// #### 論理演算子の利用
#[allow(dead_code)]
pub fn symbol(x: i32 , y: i32){
    let mut result = (x == 10) && (y == 6);
    println!("(x == {}) && (y == {}) = {}" , 10 , 6 , result);
    result = (x == 10) || (y == 20);
    println!("(x == {}) || (y == {}) = {}" , 10 , 20 , result);
    result = !(x == 10);
    println!("!(x == {}) = {}" , 10 , result);
}
