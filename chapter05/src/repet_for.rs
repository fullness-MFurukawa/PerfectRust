//!
//! 5章制御式 サンプルプログラム
//!

/// ### 5-3.forループ
/// #### リスト5-9 カウンタのインクリメントを利用したfor式
#[allow(dead_code)]
pub fn for_1(){
    println!("0～4までループする");
    for i in 0..5 {     // i < 5 (0～4までループする)
        print!("i = {} " , i);
    }
    println!("\n");
    println!("0～5までループする");
    for i in 0 ..= 5 {  // i <= 5 (0～5までループする)
        print!("i = {} " , i);
    }
}    

/// ### 5-3.forループ
/// #### リスト5-10 カウンタのデクリメントを利用したfor式
#[allow(dead_code)]
pub fn for_2(){
   println!("4～0までループする");
   for i in (0..5).rev() {      // i > 5 (4～0までループする)
       print!("i = {} " , i);
   }
   println!("\n");
   println!("5～0までループする");
   for i in (0 ..= 5).rev() {   // i >= 5 (5～0までループする)
       print!("i = {} " , i);
   }
}

/// ### 5-3.ループ
/// #### リスト5-11 要素集合から値を取得する
#[allow(dead_code)]
pub fn for_3(){
    // i32型のベクタを生成する
    let values = vec![100 ,200 ,300 , 400 , 500];
    let mut result:i32 = 0;
    for value in values {
        result += value;
    }
    println!("result = {}" , result);
}


