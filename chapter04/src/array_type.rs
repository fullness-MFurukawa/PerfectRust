//!
//! 4章 基本データ型
//! サンプルプログラム
//! 

/// ### 4-5.配列型
/// #### リスト4-13 配列の宣言
#[allow(dead_code)]
pub fn declare() {
    let array_a  = [1 , 2 , 3];    // 初期値のみ指定
    let array_b: [i32 ; 3] = [10 , 20 , 30]; // 型、要素数を指定
    let array_c = [0 ; 3]; // 初期値と要素数を指定
    println!("array_a = {:?}" , array_a);
    println!("array_b = {:?}" , array_b);
    println!("array_c = {:?}" , array_c);
}

/// ### 4-5.配列型
/// #### リスト4-14 配列の利用
#[allow(dead_code)]
pub fn use_array() {
    let mut array_a = [0 ; 3];
    array_a[0] = 100;
    array_a[1] = 200;
    array_a[2] = 300;
    // 先頭から順に値を取り出して表示
    for value in array_a{
        println!("value = {}" , value);
    }
}

/// ### 4-5.配列型
/// #### リスト4-15 配列型の主なメソッド
#[allow(dead_code)]
pub fn methods() {
    let mut array_a:[i32 ; 5] = [2 , 3 , 5 , 4 , 1];
    println!("is_empty() = {}" , array_a.is_empty());// 空かを検証する
    println!("contains() = {}" , array_a.contains(&3));// 指定の値を取得する
    println!("first() = {}" , array_a.first().unwrap());// 先頭の値を取得する
    println!("last() = {}" , array_a.last().unwrap()); // 最後の値を取得する
    array_a.sort(); // ソートする
    println!("sort = {:?}" , array_a);
    // 2023-05-21 変更
    let mut array_b:[i32 ; 5] = [2 , 3 , 5 , 4 , 1];
    array_b.reverse(); // 要素を逆に並べ替える
    println!("reverse = {:?}" , array_b);
}

/// ### 4-5.配列型
/// #### リスト4-16 多次元配列
#[allow(dead_code)]
pub fn multidimensional() {
   // 2次元配列
   let array_a =  [[0 ; 5] ; 3];
   for sub_array in array_a{
       println!("2次元配列 sub_array = {:?}" , sub_array);
   }
   // 3次元配列
   let array_b = [[[10 ; 5] ; 3] ; 2];
   for sub_array in array_b{
       println!("3次元配列 sub_array = {:?}" , sub_array);
   }
}

