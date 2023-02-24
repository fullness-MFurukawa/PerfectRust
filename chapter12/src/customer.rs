/// 12章 スレッドと非同期処理
/// サンプルコード

use serde::Serialize;
/// ## 12-9 タスク間通信
/// ### リスト12-29 顧客情報構造体
#[derive(Debug , Clone , Serialize)]
pub struct Customer {
    name:   String, // 氏名
    email:  String  // メールアドレス
}
impl Customer {
    pub fn new(_name:String,_email:String) -> Self{
        Self{name:_name , email:_email}
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}