use std::marker::PhantomData;
use anyhow::Result;
use serde::de::DeserializeOwned;
/// 10-6.サブトレイト
/// リスト10-20 SendとSyncトレイトの追加
pub trait CsvReader<T>: Send + Sync where T:DeserializeOwned  {
    fn read(&self , file_path:&str) -> Result<Vec<T>>;
}
/// ## 10-2.ジェネリックトレイト
/// ### リスト10-7 ジェネリックトレイトの実装
#[allow(dead_code)]
#[derive(Default)]
pub struct CsvReaderImpl<T>{
    phantom: PhantomData<T>
}
impl<T> CsvReaderImpl<T> {
}
impl<T> CsvReader<T> for CsvReaderImpl<T>  where Self:Send + Sync , T:DeserializeOwned {
    /// ## 10-3.メソッドの実装
    /// ## リスト10-10 CSFのデシリアライズ
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Reult<Vec<T>> 読取り結果
    fn read(&self , _file_path:&str) -> Result<Vec<T>>{
       todo!()
    }
}