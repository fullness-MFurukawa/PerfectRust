//!
//!  7章 関数 サンプルプログラム
//! 


/// ## 7-6.ライフタイム
/// ### リスト7-26 ライフタイムの基本
#[allow(dead_code)]
pub fn life_time_1(){
    // 生成
    let x = vec![ 1 , 2 , 3];   // ベクタを生成する
    let a = String::from("ABC");  // 文字列を生成する
    // 参照の取得
    let y = &x; // xの参照を変数yに代入する
    let b   = &a; // aの参照を変数bに代入する
    // 参照の利用
    println!("y = {:?}" , y); // yが参照する値を出力する
    println!("b = {:?}" , b); // bが参照する値を出力する
    println!("b = {:?}" , b); // bが参照する値を出力する
    println!("プログラム終了");
}

/// ## 7-6.ライフタイム
/// ### リスト7-27 参照を別な変数へ代入する
#[allow(dead_code)]
pub fn life_time_2(){
    // 生成
    let a = String::from("ABC");  // 文字列を生成する
    // 参照の取得
    let b = &a; // aの参照を変数bに代入する
    let c = b;  // 変数bの参照をcに代入する
    // 参照の利用
    println!("c = {:?}" , c); // cが参照する値を出力する
    println!("プログラム終了");
}

/* 
/// ## 7-6.ライフタイム
/// ### リスト7-28 参照を返す関数
#[allow(dead_code)]
pub fn life_time_3() -> &String{
    let x = String::from("ABC");
    &x
}
*/


/* 
/// ## 7-6.ライフタイム
/// ### リスト7-29 引数の参照を返す関数
#[allow(dead_code)]
fn compare(value1: &String, value2: &String) -> &String {
    // 参照で受け取った2つの文字列の長さを比較する
    if value1.len() > value2.len(){ 
        value1
    }else{
        value2
    }
}

/// ## 7-6.ライフタイム
/// ### リスト7-29 引数の参照を返す関数
#[allow(dead_code)]
pub fn life_time_4() {
    let a = String::from("ABC");
    let b = String::from("DEFG");
    // 関数に文字列の参照を渡して比較結果を受け取る
    let r = compare_1(&a , &b);
    //let r = compare_2(a.clone(), b.clone());
    println!("r = {:?}" , r);
    println!("a = {:?}" , a);
    println!("b = {:?}" , b);
}
*/

/// ## 7-7.ライフタイム注釈
/// ### リスト7-30 ライフタイム注釈の利用
#[allow(dead_code)]
fn compare<'a>(value1: &'a String, value2: &'a String) -> &'a String {
    // 参照で受け取った2つの文字列の長さを比較する
    if value1.len() > value2.len(){ 
        value1
    }else{
        value2
    }
}

/// ## 7-7.ライフタイム注釈
/// ### リスト7-31 関数実行にも注意
#[allow(dead_code)]
pub fn life_time_5() {
    let r;
    let a = String::from("ABC");
    {
        let b = String::from("DEFG");
        // 関数に文字列の参照を渡して比較結果を受け取る
        r = compare(&a , &b);
        println!("r = {:?}" , r);
    }
}

/// ## 7-7.ライフタイム注釈
/// ### リスト7-32 ライフタイム原則
/// ### 引数 Vec<i32>のミュータブルな参照
/// ### 戻り値 Vec<i32>の参照
#[allow(dead_code)]
pub fn push(value: &mut Vec<i32>) -> &Vec<i32> {   
    value.push(10);
    value.push(11);
    value.push(12);
    value
}
    