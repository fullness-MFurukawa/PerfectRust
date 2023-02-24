///
/// 11章エラー サンプルコード
/// 

use std::fs::File;
use std::env::current_dir;
/// ## 11-4.パニック
/// ### リスト11-11 expect()メソッド
#[allow(dead_code)]
pub fn use_expect() {
    // 存在しないファイルパスを生成
    let file_path = current_dir()
        .map(|path_buf| path_buf.join("\\files\\param.txt"))
        .map_err(|error| panic!("{}" , error)).unwrap();
    // 存在しなファイルを開く
    let _file= File::open(file_path).expect("ファイルが存在しないので処理を終了します。");
}

/// ## 11-4.パニック
/// ### リスト11-12 panic!()マクロ
#[allow(dead_code)]
pub fn use_panic() {
    // 存在しないファイルパスを生成
    let file_path = current_dir()
        .map(|path_buf| path_buf.join("\\files\\param.txt"))
        .map_err(|error| panic!("{}" , error)).unwrap();
    // 存在しなファイルを開く
    let file= File::open(file_path);
    if file.is_err() {
        panic!("{},{:?}" ,"ファイルが存在しないので処理を終了します。",
         file.err().unwrap());
    }
}