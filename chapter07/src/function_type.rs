//!
//!  7章 関数 サンプルプログラム
//! 

/// ## 7-2.関数型
/// ### リスト7-8 加算関数
#[allow(dead_code)]
fn add(x: i32 , y: i32) -> i32 {
    x + y
}
/// ## 7-2.関数型
/// ### リスト7-8 関数を変数へ代入して利用
#[allow(dead_code)]
pub fn use_function_1() {
    let func = add; // 変数funcにadd()関数を代入する
    let r = func(10 , 20); // 変数を利用してadd()関数を呼び出す 
    println!("x + y = {}" , &r); 
}

type Calc = fn(i32 , i32) -> i32; // i32型引数を2つ受け取り、i32の結果を返す関数の型
/// ## 7-2.関数型
/// ### リスト7-9 typeキーワードを利用する
/// ### 引数 func:Calc  Calc型関数
#[allow(dead_code)]
fn use_calc_type(func: Calc , x: i32 , y: i32) -> i32 {
    func(x , y) // 引数で渡された関数の呼び出し結果を返す
}
/// ## 7-2.関数型
/// ### リスト7-9 typeキーワードを利用する
#[allow(dead_code)]
pub fn use_function_2() {
    let calc:Calc = add; // Calc型変数にadd()関数を代入する
    let r = use_calc_type(calc , 10 , 20); // 引数で関数を渡す
    println!("x + y = {}" , &r);
}

/// ## 7-2.関数型
/// ### リスト7-10 関数型を返す
/// ### 戻り値　Cald
#[allow(dead_code)]
fn return_calc_type() -> Calc {
    add // add()関数をリターンする
}    
/// ## 7-2.関数型
/// ### リスト7-10 関数型を返す
#[allow(dead_code)]
pub fn use_function_3() {
    let calc = return_calc_type(); // 関数型Calcを受け取る
    let r = calc(10 ,20);// 受け取った関数を実行する
    println!("10 + 20 = {}" , &r);
}

