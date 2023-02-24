//!
//! # ３章 変数と定数
//! サンプルプログラム
//! 


///  ### 3-2.静的変数
///  #### リスト3-6 消費税率を表す静的変数
static TAX_RATE:f32 = 0.10;
///  ### 3-2.静的変数
///  #### リスト3-6 静的変数の宣言と利用 
///  #### 引数:price 単価
///  #### 戻り値:消費税込みの金額
#[allow(dead_code)] 
pub fn calc_amount(price: i32) -> i32 {
    let fprice = price as f32; // i32型の値をf32型に変換する
    let result = fprice + fprice * TAX_RATE; // 消費税込みの金額を計算する
    result as i32 // f32型からi32型に変換して結果を返す
}

///  ### 3-2.静的変数
///  #### リスト3-8 unsafeブロックの利用
#[allow(dead_code)] 
static mut TOTAL_VALUE:i32 = 0;
///  ### 3-2.静的変数
///  #### リスト3-8 unsafeブロックの利用
#[allow(dead_code)] 
pub fn calc_total(value: i32) {
    unsafe{
        TOTAL_VALUE += value;
        println!("TOTAL_VALUE = {}" , TOTAL_VALUE);
    }
}
