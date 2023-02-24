//!
//! 4章 基本データ型
//! サンプルプログラム
//! 

/// ### 4-3.論理型
/// #### リスト4-8 論理型の利用
#[allow(dead_code)]
pub fn is_adult(age: i32) -> bool {
    println!("age = {}" , age);
    if age > 19 {
        true  // 成人
    }else{
        false // 未成年
    }
}

/// ### 4-3.論理型
/// #### リスト4-9 論理型のメソッド
#[allow(dead_code)]
pub fn method(age: i32) -> String {
    let result:Option<String> = true.then(||{
        if age > 19 {
            format!("{}歳は成人です。" , age)
        }else{
            format!("{}歳は未成年です。" , age)
        }
    });
    result.unwrap()
}

