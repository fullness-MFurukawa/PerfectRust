///
/// 10章トレイト サンプルコード
/// 

use crate::association::entities::Product;
use crate::association::traits::{CsvReader, JsonReader};
use crate::association::traits_impl::{CsvReaderImpl ,JsonReaderImpl};

/// ## 10-4.関連型トレイト(Association Type)
/// ### リスト10-15 メソッドの実行
#[allow(dead_code)]
pub fn use_association_method() {
    // ファイルパスを生成する
    let csv_path = concat!(env!("CARGO_MANIFEST_DIR"), "\\resources\\products.csv");
    let json_path= concat!(env!("CARGO_MANIFEST_DIR"), "\\resources\\products.json");
    // インスタンスを生成する
    let csv_method = CsvReaderImpl::<Product>::default();
    let json_method = JsonReaderImpl::<Product>::default();
    // メソッドを実行する
    let csv_result = csv_method.read(csv_path).unwrap();
    let json_result = json_method.read(json_path).unwrap();
    // 結果を出力
    println!("<<CSV>>");
    for result in csv_result {
        println!("{:?}" , result);
    }
    println!("<<JSON>>");
    for result in json_result {
        println!("{:?}" , result);
    }
}