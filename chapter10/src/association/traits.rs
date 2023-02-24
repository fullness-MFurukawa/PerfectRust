///
/// 10章トレイト サンプルコード
/// 

use anyhow::Result;
/// ## 10-4.Association Type
/// ### リスト10-13 CSV形式のファイルを読み取る機能を表すトレイト
/// ### T:読取り結果を格納する構造体
pub trait CsvReader {
    type Entity; // Association Typeの宣言
    /// ## 読取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Reult<Vec<Entity>> 読取り結果
    fn read(&self , file_path:&str) -> Result<Vec<Self::Entity>>;
}
/// ## 10-4.Association Type
/// ### リスト10-13 JSON形式のファイルを読み取る機能を表すトレイト
/// ### T:読取り結果を格納する構造体
/// 
pub trait JsonReader {
    type Entity; // Association Typeの宣言
    /// ## 読取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Reult<Vec<T>> 読取り結果
    fn read(&self , file_path:&str) -> Result<Vec<Self::Entity>>;
}