

use std::path::PathBuf;
use std::fs::{File,read_to_string};
use std::io::BufReader;
use std::marker::PhantomData;
use csv::ReaderBuilder;
use anyhow::Result;
use serde::de::DeserializeOwned;
use crate::association::traits::{CsvReader,JsonReader};

/// ## 10-4.Association Type
/// ### リスト10-14 関連型トレイトの実装
#[derive(Default)]
pub struct CsvReaderImpl<T>{
    phantom: PhantomData<T>
}
impl<T> CsvReader for  CsvReaderImpl<T> where T:DeserializeOwned {
    type Entity = T; // 関連型に構造体のジェネリック型を指定する
    /// ## 読取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Reult<Vec<Entity>> 読取り結果
    fn read(&self , file_path:&str) -> Result<Vec<Self::Entity>>{
        let path_buf = PathBuf::from(file_path); // PathBufを生成する
        let string_data = read_to_string(path_buf)?; // 文字列データを読み取る
        // バイナリ形式に変換する
        let mut reader = ReaderBuilder::new().delimiter(b',').from_reader(string_data.as_bytes());
        // データを指定された型に変換する
        let rows = reader.deserialize::<T>();
        // ベクタに格納する
        let mut result = Vec::<T>::new();
        for row in rows {
            result.push(row?);
        }
        Ok(result)
    }
}
/// ## 10-4.Association Type
/// ### リスト10-14 関連型トレイトの実装
#[derive(Default)]
pub struct JsonReaderImpl<T>{
    phantom: PhantomData<T>
}
impl<T> JsonReader for JsonReaderImpl<T> where T:DeserializeOwned {
    type Entity = T; // 関連型に構造体のジェネリック型を指定する
    /// ## 読取り
    /// ### 引数 file_path:ファイルパス
    /// ### 戻り値 Reult<Vec<T>> 読取り結果
    fn read(&self , file_path:&str) -> Result<Vec<Self::Entity>>{
        let path_buf = PathBuf::from(file_path); // PathBufを生成する
        // BufReaderを生成する
        let buf_reader = File::open(path_buf)
        .map(|file| BufReader::new(file))?;
        let result = serde_json::from_reader(buf_reader)?;  // デシリアライズする
        Ok(result)
    }
}

#[cfg(test)]
mod tests{
    use crate::association::entities::Product;
    use super::*;
    #[test]
    fn csv_reader() -> Result<()>{
        let file_name =     
        concat!(env!("CARGO_MANIFEST_DIR"), "\\resources\\products.csv");
        let csv = CsvReaderImpl::<Product>::default();
        let result = csv.read(file_name)?;
        println!("{:?}", result);
        Ok(())
    }

    #[test]
    fn json_reader() -> Result<()>{
        let file_name =     
        concat!(env!("CARGO_MANIFEST_DIR"), "\\resources\\products.json");
        let json = JsonReaderImpl::<Product>::default();
        let result = json.read(file_name)?;
        println!("{:?}" , result);
        Ok(())
    }
}
