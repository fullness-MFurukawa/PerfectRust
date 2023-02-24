//!
//! 5章制御式 サンプルプログラム
//!

/// ### 5-4.whileループ
/// #### リスト5-12 基本的なwhile式
#[allow(dead_code)]
pub fn while_1(){
    let mut counter = 1;
    while counter < 5 {
        if counter % 2 == 0{
            println!("counterの値{}は偶数です。" , counter);
        }else{
            println!("counterの値{}は奇数です。" , counter);
        }
        counter += 1;
    }
}    

/// ### 5-4.whileループ
/// #### リスト5-13 while-let式
#[allow(dead_code)]
pub fn while_2(){
    let x = ["ABC" , "ABC" , "ABC" , "XYZ"];
    let mut counter = 0;
    // スライスの値が"ABC"の間繰り返す
    while let "ABC" = x[counter]{
        println!("x[counter] = {}" , x[counter]);
        counter += 1;
    }
}
