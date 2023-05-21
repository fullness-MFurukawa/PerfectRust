//!
//! 4章 基本データ型
//! 
mod integer_type;
mod floating_type;
mod boolean_type;
mod character_type;
mod array_type;
mod tuple_type;
mod slice_type;
mod str_type;
mod pointer_type;
fn main() {
    //integer_type::integer_literal(); // 整数リテラル
    //integer_type::byte_literal();   // バイトリテラル
    //integer_type::i32_constant();     // i32型の定数
    //integer_type::methods();    //  整数型の主なメソッド
    //floating_type::floating_literal(); // 浮動小数点リテラル
    //floating_type::floating_constant(); // 浮動小数点型の定数
    //floating_type::methods(); // 浮動小数点型の主なメソッド
    //let result = boolean_type::is_adult(30); // 論理値の利用
    //println!("{}" , result);
    //println!("{}" , boolean_type::method(30));
    //character_type::char_literal(); // 文字型リテラル
    //character_type::char_constant(); // 文字型定数
    //character_type::methods(); // 文字型のメソッド
    //array_type::declare(); // 配列の宣言
    //array_type::use_array(); // 配列の利用
    array_type::methods(); // 配列のメソッド
    //array_type::multidimensional(); // 多次元配列
    //tuple_type::declare(); // タプル型の宣言
    //tuple_type::calc((20 , 5)); // インデックスの利用
    //tuple_type::methods(); // タプル型の主なメソッド
    //slice_type::get(); // スライスの取得
    //slice_type::range();    //   Range構造体を利用する
    //slice_type::multibyte_slice(); // マルチバイトの利用
    //slice_type::fat_pointer(); // ファットポインタを確認する
    //slice_type::methods_1();    // 値の取得、状態確認メソッド
    //slice_type::methods_2(); // ソートメソッド
    //slice_type::methods_3(); // データ変換、加工メソッド
    //str_type::declar(); // 文字列リテラル
    //str_type::binding(); // 文字列の結合
    //str_type::methods_1(); // データ変換メソッド
    //str_type::methods_2(); // 値を調べるメソッド
    //str_type::methods_3(); // 値を取得する
    //str_type::methods_4();// 文字列を生成、置換するメソッド
    //str_type::methods_5(); // 大文字/小文字変換、トリミング、分割メソッド
    //pointer_type::declare(); // ポインタ型の宣言
    //pointer_type::mut_declare_1(); // 値の変更
    //pointer_type::mut_declare_2();// 可変ポインタ
}
