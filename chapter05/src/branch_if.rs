//!
//! 5章制御式 サンプルプログラム
//! 

/// ### 5-1.条件分岐
/// #### リスト5-1 if-else式
#[allow(dead_code)]
pub fn branch_1(){
    let name = "山田太郎";
    if name.eq("山田太郎") {
        println!("変数nameの値は「山田太郎」です。");
    }
    let num = 120;
    if num % 2 == 0 {
        println!("変数numの値は偶数です。");
    }else{
        println!("変数numの値は奇数です。");        
    }
}
/// ### 5-1.条件分岐
/// #### リスト5-2 if-else-if式
#[allow(dead_code)]
pub fn branch_2(){
    let num = 10;
    if num == 1 {
        println!("変数numの値は1です。");
    }else if num == 2 {
        println!("変数numの値は2です。");
    }else{
        println!("変数numの値は1でも2でもありません。");
    }
}
/// ### 5-1.条件分岐
/// #### リスト5-3 if-let式
#[allow(dead_code)]
pub fn branch_3(){
    let num = 10;
    let result = if num == 1 {
        "変数numの値は1です。"
    }else if num == 2 {
        "変数numの値は2です。"
    }else{
        "変数numの値は1でも2でもありません。"
    };
    println!("{}" , result);
}
