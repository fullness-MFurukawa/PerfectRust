//!
//!  7章 関数 サンプルプログラム
//!  


/// ## 7-5.参照
/// ### リスト7-22 参照の取得
/*
#[allow(dead_code)]
pub fn reference_1(){
    let x = Vec::<i32>::new(); // イミュータブルベクタを生成する
    let y = &mut x; // 変数xのミュータブルな参照を代入する
    y.push(1); // 値を追加する
    y.push(2); // 値を追加する
    y.push(3); // 値を追加する
    println!("{:?}" , y); // 値を出力する
}
*/
/// ## 7-5.参照
/// ### リスト7-23 参照の取得
#[allow(dead_code)]
pub fn reference_2(){
    let mut x = Vec::<i32>::new(); // イミュータブルベクタを生成する
    let y = &mut x; // 変数xのミュータブルな参照を代入する
    y.push(1); // 値を追加する
    y.push(2); // 値を追加する
    y.push(3); // 値を追加する
    println!("{:?}" , y); // 値を出力する
}
/* 
/// ## 7-5.参照
/// ### リスト7-24 参照の混在はできない
#[allow(dead_code)]
pub fn reference_3(){
    let mut x = Vec::<i32>::new(); // イミュータブルベクタを生成する
    let y = &mut x; // 変数xの参照を代入する
    let z = &mut x; // 変数xの参照を代入する
    y.push(100);
    z.push(200);
    println!("{:?}" , x);
}
*/
/* 
/// ## 7-5.参照
/// ### リスト7-25 参照は混在できない
#[allow(dead_code)]
pub fn reference_4(){
    let mut x = Vec::<i32>::new(); // イミュータブルベクタを生成する
    let y = &mut x; // 変数xの参照を代入する
    let z = &x; // 変数xの参照を代入する
    y.push(100);
    println!("{:?}" , x);
}
*/