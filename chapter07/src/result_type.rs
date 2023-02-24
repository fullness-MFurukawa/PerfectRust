//!
//!  7章 関数 サンプルプログラム
//!


use std::result::Result;

/// ## 7-9.Result<T ,E>
/// ### リスト7-43 変数宣言
#[allow(dead_code)]
pub fn value_setting() {
    let mut x:Result<i32 , String>; // Okにi32型、ErrにString型を保持するResult型
    x = Ok(100); // Okに100を設定する
    println!("x = {:?}" , x);
    x = Err(String::from("エラーです。")); // Errに文字列を設定する
    println!("x = {:?}" , x);
}


/// ## 7-9.Result<T,E>
/// ### リスト7-44 関数での利用
#[allow(dead_code)]
fn div(value1: i32 , value2: i32) -> Result<i32,String> {
    if value2 == 0 {
        return Err("ゼロ除算です。".to_owned());
    }
    Ok((value1 / value2) as i32)
}
// ## 7-9.Result<T , E>
/// ### リスト7-44 関数での利用
#[allow(dead_code)]
pub fn use_div() {
    let x = 10;
    let y = 5;
    // div()関数の実行結果を変数rで受け取る
    let r = match div(x , y) {
        Ok(result) => format!("{} / {} = {}" , x , y , result) ,
        Err(err) => err
    };
    println!("{}" , r);
}

/// ## 7-9.Result<T , E>
/// ### リスト7-45 値の検証メソッド
#[allow(dead_code)]
pub fn method_1() {
    let x = 10;
    let y = 0;
    let result = div(x, y);
    let r = if result.is_ok() { // 結果はOkか?
        format!("{} / {} = {}" , x , y , result.unwrap())
    }else{
        result.unwrap_err()
    };
    println!("{}" , r);
}

/// ## 7-9.Result<T , E>
/// ### リスト7-46 値の取得メソッド
#[allow(dead_code)]
pub fn method_2() {
    let x = 10;
    let y = 0;
    let r = div(x, y).unwrap_or(-100); 
    println!("unwrap_or() = {}" , r);
    let r = div(x, y)
    .unwrap_or_else(|e| {
        println!("{:?}" , e);
        -100
    } ); 
    println!("unwrap_or_else() = {}" , r);
    let r = div(x, y).unwrap_or_default();
    println!("unwrap_or_default() = {}" , r);
}

/// ## 7-9.Result<T , E>
/// ### リスト7-47 コンビネータ
#[allow(dead_code)]
pub fn method_3() {
    let x = 10;
    let y = 5;
    let r = div(x, y).and_then(|result| Ok(result * 2));
    println!("and_then() = {:?} " , r);
    let r = div(x, y).map(|result| result * 2);
    println!("map() = {:?} " , r);
    let x = 10;
    let y = 0;
    let r = div(x, y).map_err(|error| error);
    println!("map_err() = {:?} " , r);
    let r = div(x, y).map_or(-100, |result| result);
    println!("map_or() = {:?} " , r);
    let r = div(x, y).map_or_else(|error| error , |result|  result.to_string());
    println!("map_or_else() = {:?} " , r);
    let r = div(x, y).or_else(|error| Err(error));
    println!("or_else() = {:?} " , r);
}
/// ## 7-9.Result<T , E>
/// ### リスト7-48 Option型へ変換する
#[allow(dead_code)]
pub fn method_4() {
    let x = 10;
    let y = 5;
    let r = div(x, y).ok();
    println!("ok() = {:?} " , r);
    let x = 10;
    let y = 0;
    let r = div(x, y).err();
    println!("err() = {:?} " , r);
}

/// ## 7-9.Result<T , E>
/// ### リスト7-49 ?演算子
#[allow(dead_code)]
pub fn method_5() -> Result<String ,String>{
    let x = 10;
    let y = 5;
    let r = div(x, y)?;
    Ok(format!("{} / {} = {}" , x , y , r))
}