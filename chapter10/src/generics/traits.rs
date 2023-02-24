///
/// 10章トレイト サンプルコード
/// 

use anyhow::Result;
use serde::de::DeserializeOwned;
/// ## 10-2.ジェネリックトレイト
/// ### リスト10-6 CSV形式のファイルを読み取る機能を表すトレイト
/// ### T:読取り結果を格納する構造体
pub trait CsvReader<T> where T:DeserializeOwned {
    /// ## 読取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Reult<Vec<T>> 読取り結果
    fn read(&self , file_path:&str) -> Result<Vec<T>>;
}
/// ## 10-2.ジェネリックトレイト
/// ### リスト10-6 JSON形式のファイルを読み取る機能を表すトレイト
/// ### T:読取り結果を格納する構造体
/// 
pub trait JsonReader<T> where T:DeserializeOwned {
    /// ## 読取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Reult<Vec<T>> 読取り結果
    fn read(&self , file_path:&str) -> Result<Vec<T>>;
}