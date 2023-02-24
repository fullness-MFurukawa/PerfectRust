//!
//!  7章 関数 サンプルプログラム
//!

/// ## 7-8.Option<T>
/// ### リスト7-34 変数宣言
#[allow(dead_code)]
pub fn declare() {
    let mut a = None;
    println!("a = {:?}" , a);
    a = Some(100);
    println!("a = {:?}" , a);
    let mut b = Some( String::from("ABCD") );
    println!("b = {:?}" , b);
    b = None;
    println!("b = {:?}" , b);
}

/// ## 7-8.Option<T>
/// ### リスト7-35 関数での利用
#[allow(dead_code)]
fn div(value1: i32 , value2: i32) -> Option<i32> {
    if value2 == 0 { //　0除算になる
        return None; // Noneを返す
    }
    let r = (value1 / value2) as i32;
    Some(r) // 割り算の結果を返す
}
/// ## 7-8.Option<T>
/// ### リスト7-35 関数での利用
#[allow(dead_code)]
pub fn use_div() {
    let x = 10;
    let y = 5;
    // div()関数の実行結果を変数rで受け取る
    let r = match div(x, y){ 
        None => "割り算できません。".to_owned() , 
        Some(result) => format!("{} / {} = {}", x , y , result)
    };
    println!("{}" , r);//関数実行結果を出力する
}

/// ## 7-8.Option<T>
/// ### リスト7-36 値の評価メソッド
#[allow(dead_code)]
pub fn method_1() {
    let x = 10;
    let y = 0;
    let result = div(x, y);
    let r = if result.is_some(){ // 結果はSomeか?
        format!("{} / {} = {:?}" , x , y , result.unwrap())
    }else{
        "割り算できません。".to_owned()
    };
    println!("{}" , r);
}

/// ## 7-8.Option<T>
/// ### リスト7-37 値の取得メソッド
#[allow(dead_code)]
pub fn method_2() {
    let x = 10;
    let y = 0;
    let r = div(x, y).unwrap_or(-1);
    println!("unwrap_or() = {}" , r);
    let r = div(x, y).unwrap_or_else(|| -100);
    println!("unwrap_or_else() = {}" , r);
    let r = div(x, y).unwrap_or_default();
    println!("unwrap_or_default() = {}" , r);
}


/// ## 7-8.Option<T>
/// ### リスト7-38 コンビネータ
#[allow(dead_code)]
pub fn method_3() {
    let x = 10;
    let y = 5;
    let r = div(x, y)
    .and_then(|result| Some(format!("{} / {} = {}", x, y, result)));
    println!("and_then() = {:?} " , r.unwrap());
    let r = div(x,y)
    .map(|result| format!("{} / {} = {}", x, y, result));
    println!("map() = {:?} " , r.unwrap());
    let x = 10;
    let y = 0;
    let r = div(x, y)
    .map_or("計算できません。".to_owned() , 
    |result| format!("{} / {} = {}", x, y, result));
    println!("map_or() = {:?} " , r);
    let r = div(x, y)
    .map_or_else(|| "計算できません。".to_owned() , 
    |result| format!("{} / {} = {}", x, y, result));
    println!("map_or_else() = {:?} " , r);
}

/// ## 7-8.Option<T>
/// ### リスト7-39 Result型への変換メソッド
#[allow(dead_code)]
pub fn method_4(){
    let x = 10;
    let y = 5;
    let r = match div(x, y).ok_or("計算できません。".to_owned()){
        Ok(result) => format!("{} / {} = {}" , x , y , result) ,
        Err(e) => e
    };
    println!("ok_or() = {:?} " , r);
    let x = 10;
    let y = 0;
    let r = match div(x, y).ok_or_else(|| "計算できません。".to_owned()){
        Ok(result) => format!("{} / {} = {}" , x , y , result) ,
        Err(e) => e
    };
    println!("ok_or_else() = {:?} " , r);
}

/// ## 7-8.Option<T>
/// ### リスト7-40 ?演算子
#[allow(dead_code)]
pub fn method_5() -> Option<String>{
    let x = 10;
    let y = 5;
    let result:i32 = div(x,y)?;
    Some(format!("{} / {} = {}", x, y, result))
}