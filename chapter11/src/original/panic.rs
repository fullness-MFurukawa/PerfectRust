

/// ## 11-1.エラー型の基本
/// ### リスト11-2 std::io::ErrorKind列挙型の利用
#[allow(dead_code)]
pub fn use_error_kind() {
    // 存在しないファイルパスを生成
    let file_path = current_dir()
        .map(|path_buf| path_buf.join("\\files\\sample.txt"))
        .map_err(|error| panic!("{}" , error)).unwrap();
    // 存在しなファイルを開く
    let error = File::open(file_path).err().unwrap();
    // エラーの種類を出力する
    let result = match error.kind(){
        ErrorKind::NotFound => "指定されたファイルが見つかりません。" ,
        ErrorKind::PermissionDenied => "指定されたファイルへの操作権限がありません。" ,
        _ => "判別できないエラーが発生しました。" 
    };
    println!("{}" , result);
}