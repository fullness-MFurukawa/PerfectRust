//!
//!  7章 関数 サンプルプログラム
//!  

/// ## 7-4.所有権
/// ### リスト7-15 代入による所有権の移動
/* 
#[allow(dead_code)]
pub fn ownership_1() {
    // 文字列を生成して変数xへ代入
    let x = String::from("ABC");// 変数xがインスタンスの所有権を持っている
    println!("x = {:?}" , x);   // 変数xの値を出力
    // 変数xの値を変数yへ代入
    let y = x;
    println!("x = {:?}" , x);   // 変数xの値を出力
    println!("y = {:?}" , y);   // 変数yの値を出力
}
*/

/// ## 7-4.所有権
/// ### リスト7-16 代入による所有権の移動
#[allow(dead_code)]
pub fn ownership_2() {
    // 文字列を生成して変数xへ代入
    let x = String::from("ABC");// 変数xがインスタンスの所有権を持っている
    println!("x = {:?}" , x);   // 変数xの値を出力
    let y = &x;        // 変数xの参照を変数yへ代入
    println!("x = {:?}" , x);   // 変数xの値を出力
    println!("y = {:?}" , y);   // 変数yの値を出力
}

/// ## 7-4.所有権
/// ### リスト7-17 代入による所有権の移動
#[allow(dead_code)]
pub fn ownership_3() {
    // 文字列を生成して変数xへ代入
    let x = String::from("ABC");// 変数xがインスタンスの所有権を持っている
    println!("x = {:?}" , x);   // 変数xの値を出力
    let y = x.clone();  // close()メソッドで変数xの複製を作成して代入する
    println!("x = {:p}" , &x);  // 変数xの値を出力
    println!("y = {:p}" , &y);  // 変数yの値を出力
}

/// ## 7-4.所有権
/// ### リスト7-18 引数渡しによる所有権の移動
/* 
#[allow(dead_code)]
fn print_message(message: String){
    println!("message = {:?}" , message);
}
*/
/// ## 7-4.所有権
/// ### リスト7-18 引数渡しによる所有権の移動
/* 
#[allow(dead_code)]
pub fn ownership_4() {
    // 文字列を生成して変数xへ代入
    let x = String::from("ABC");// 変数xがインスタンスの所有権を持っている
    print_message(x); // print_message()関数に変数xを渡す
    println!("x = {:?}" , x);
}
*/
/// ## 7-4.所有権
/// ### リスト7-19 引数渡しによる所有権の移動
#[allow(dead_code)]
fn print_message(message: &String){
    println!("message = {:?}" , message);
}
/// ## 7-4.所有権
/// ### リスト7-19 引数渡しによる所有権の移動
#[allow(dead_code)]
pub fn ownership_5() {
    // 文字列を生成して変数xへ代入
    let x = String::from("ABC");// 変数xがインスタンスの所有権を持っている
    print_message(&x); // print_message()関数に変数xを渡す
    println!("x = {:?}" , x);
}

/// ## 7-4.所有権
/// ### リスト7-20 リターンによる所有権の移動
#[allow(dead_code)]
fn message() -> String {
    let r = String::from("good morning");
    r
}
/// ## 7-4.所有権
/// ### リスト7-20 リターンによる所有権の移動
#[allow(dead_code)]
pub fn ownership_6() {
    let x = message(); // message()関数の実行
    println!("x = {:?}" , x);
}


/* 
/// ## 7-4.所有権
/// ### リスト7-21 スコープ
#[allow(dead_code)]
pub fn ownership_7() {
    {
        // 文字列を生成して変数xへ代入
        let x = String::from("ABC");// 変数xがインスタンスの所有権を持っている
        println!("x = {:?}" , x);
    }
    let y = &x;
    println!("y = {:?}" , y);
}
*/