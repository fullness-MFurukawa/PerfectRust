//!
//! 6章 標準ライブラリ
//! 

mod string_type;
mod date_time_type;
mod box_type;
mod vec_type;
mod linked_list_type;
mod hash_map_type;
mod hash_set_type;
fn main() {
    string_type::instantiate();     // インスタンスを生成する
    string_type::add();             // 文字、文字列の追加する
    string_type::replace();         // 文字、文字列を置換する
    string_type::remove();          // 文字、文字列を削除する
    string_type::find();            // 検索
    string_type::matching();        // マッチング
    string_type::get();             // 文字、文字列を取得する
    date_time_type::instantiate();  // 現在の日時を取得する
    date_time_type::format();       // 日時のフォーマット変換
    date_time_type::from_string();  // 文字列から日時に変換する
    date_time_type::get();          // 日時構成要素を取得する
    date_time_type::change();       // 日時構成要素を変更する
    date_time_type::time_zone();    // タイムゾーンを利用する
    date_time_type::unix_epoch();   // UNIXエポックを取得する
    box_type::instantiate();        // インスタンスを生成する
    vec_type::instantiate();        // Vec<T>型の生成と利用
    vec_type::add();                // 要素の追加
    vec_type::get_and_change();     // 要素の取得と変更
    vec_type::remove();             // 要素の削除
    vec_type::sort();               // 要素をソートする
    linked_list_type::instantiate();// インスタンスを生成する
    linked_list_type::add();        // 要素を追加する
    linked_list_type::change();     // 要素を変更する
    linked_list_type::remove();     // 要素を削除する
    hash_map_type::instantiate();   // インスタンスを生成する
    hash_map_type::add();           // 要素を追加する
    hash_map_type::get_and_change();// 要素の取得と変更
    hash_map_type::remove();        // 要素の削除
    hash_set_type::instantiate();   // インスタンスを生成する
    hash_set_type::add_and_remove();// 要素の追加と削除
    hash_set_type::get();           // 要素を取得する
    hash_set_type::set_operation(); // 集合演算
}
