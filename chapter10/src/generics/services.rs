
///
/// 10章トレイト サンプルコード
/// 

use anyhow::Result;
use serde::de::DeserializeOwned;
use crate::generics::traits_impl::{CsvReaderImpl , JsonReaderImpl};
use crate::generics::traits::{CsvReader , JsonReader};
///
/// ## 10-5. 抽象化
/// ### リスト10-17 トレイトを集約する構造体
pub struct ReadService<T> {
    csv_reader:  Box<dyn CsvReader<T>> , // CsvRaederトレイト実装型フィールド
    json_reader: Box<dyn JsonReader<T>>  // JsonRaederトレイト実装型フィールド
}
impl<T:DeserializeOwned + 'static> ReadService<T>{
    /// ## 10-5.抽象化
    /// ### リスト10-17 new()関数とメソッドの実装
    /// ### コンストラクタ
    pub fn new() -> Self {
        Self {
            // CsvReader実装構造体のインスタンスをセットする
            csv_reader:  Box::new(CsvReaderImpl::<T>::new())  as Box<dyn CsvReader<T>> ,
            // JsonReader実装構造体のインスタンスをセットする
            json_reader: Box::new(JsonReaderImpl::<T>::new()) as Box<dyn JsonReader<T>>
        }
    }
    /// ## 10-5.抽象化
    /// ### リスト10-17 new()関数とメソッドの実装
    /// ### CsvReaderのメソッドに処理を委譲する
    pub fn csv_read(&self , file_path: &str) -> Result<Vec<T>> {
        let result = self.csv_reader.read(file_path)?;
        Ok(result)
    } 
    /// ## 10-5.抽象化
    /// ### リスト10-17 new()関数とメソッドの実装
    /// ### JsonReaderのメソッドに処理を委譲する
    pub fn json_read(&self , file_path: &str) -> Result<Vec<T>> {
        let result = self.json_reader.read(file_path)?;
        Ok(result)
    }
}
#[cfg(test)]
mod tests{
    use crate::association::entities::Product;
    use super::*;
    #[test]
    fn csv_read() -> Result<()>{
        let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "\\resources\\products.csv");
        let service = ReadService::<Product>::new();
        let results = service.csv_read(csv_path)?;
        for result in results {
            println!("{:?}" , result);
        }
        Ok(())
    }        
}
