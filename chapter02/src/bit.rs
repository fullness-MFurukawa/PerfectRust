//!
//! ## ビット演算子のサンプルコード
//! 

/// ### リスト2-15
/// #### ビット演算子の利用
#[allow(dead_code)]
pub fn symbol(x: i8 , y: i8){
    println!("x & y = {:b}" , x & y); // ビットのAND演算
    println!("x | y = {:b}" , x | y); // ビットのOR演算
    println!("x ^ y = {:b}" , x ^ y); // 排他的論理和
    println!("!x = {:b}" , !x);       // ビットの反転演算
    println!("x >> 3 = {:b}" , x >> 3);// ビットの右シフト(3ビット)
    println!("y << 3 = {:b}" , y << 3);// ビットの左シフト(3ビット)
}
/// ### リスト2-16
/// #### フォーマット指定文字の利用
#[allow(dead_code)]
pub fn use_format_specifier(){
    let x = 127;
    println!("2進数     = {:b}" , x);
    println!("8進数     = {:o}" , x);
    println!("16進数    = {:x}" , x);
    println!("16進数    = {:X}" , x);
    println!("指数      = {:e}" , x);
    println!("指数      = {:E}" , x);
}
/// ### リスト2-17
/// #### ビット演算メソッドの利用
#[allow(dead_code)]
pub fn methods(x: i8 , y: i8){
    use std::ops::{BitAnd , BitOr , BitXor , Not , Shr , Shl};
    println!("x & y = {:b}" , x.bitand(y)); // ビットのAND演算
    println!("x | y = {:b}" , x.bitor(y));  // ビットのOR演算
    println!("x ^ y = {:b}" , x.bitxor(y)); // 排他的論理和
    println!("!x = {:b}" , x.not());        // ビットの反転演算
    println!("x >> 3 = {:b}" , x.shr(3));   // ビットの右シフト
    println!("y << 3 = {:b}" , y.shl(3));   // ビットの左シフト
}

/// ### リスト2-18
/// #### 複合代入演算子の利用
#[allow(dead_code)]
pub fn compound_assign(mut x: i32 , mut y: i32){
    x &= y;// ビットのAND演算
    println!("x &= y = {:b}" , x); 
    x |= y;// ビットのOR演算
    println!("x |= y = {:b}" , x); 
    x ^= y;// 排他的論理和
    println!("x ^= y = {:b}" , x); 
    x >>= 3;// ビットの右シフト
    println!("x >>= 3 = {:b}" , x); 
    y <<= 3;    // ビットの左シフト
    println!("y <<= 3 = {:b}" , y);
}

/// ### リスト2-19
/// #### 複合代入演算メソッドの利用
#[allow(dead_code)]
pub fn compound_assign_method(mut x: i32 , mut y: i32){
    use std::ops::{BitAndAssign , BitOrAssign , BitXorAssign , ShrAssign , ShlAssign};
     x.bitand_assign(y);// ビットのAND演算
     println!("x &= y -> {:b}" , x); 
     x.bitor_assign(y); // ビットのOR演算
     println!("x |= y -> {:b}" , x);
     x.bitxor_assign(y);// 排他的論理和
     println!("x ^= y -> {:b}" , x);
     x.shr_assign(3);// ビットの右シフト
     println!("x >>= 3 -> {:b}" , x);
     y.shl_assign(3);// ビットの左シフト
     println!("y <<= 3 -> {:b}" , y);
}