//!
//! 5章 制御式
//! 
mod branch_if;
mod branch_match;
mod repet_for;
mod repet_while;
mod repet_loop;
fn main() {
    //branch_if::branch_1();    // if-else式
    //branch_if::branch_2();    // if-else-if式
    //branch_if::branch_3();    // if-let式
    //branch_match::branch_1(); // match式
    //branch_match::branch_2(); // match式
    //branch_match::branch_3(); // match-let式
    //branch_match::branch_4(); // RangeとOR演算子
    //branch_match::branch_5(); // ガード
    //repet_for::for_1(); // カウンタのインクリメントを利用したfor式
    //repet_for::for_2(); // カウンタのデクリメントを利用したfor式
    //repet_for::for_3(); // 要素集合から値を取得する
    //repet_while::while_1();// while式
    //repet_while::while_2(); // while-let式
    repet_loop::loop_1(); // loop式の利用
}
