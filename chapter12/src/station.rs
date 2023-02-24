/// 12章 スレッドと非同期処理
/// サンプルコード

use serde::Deserialize;
use std::fmt::{Debug, Formatter};
// 駅名と表示メッセージを表す構造体
#[derive(Deserialize)]
pub struct Station  {
    name: String,   //  駅名フィールド
    message: String //  メッセージフィールド
}
#[allow(dead_code)]
impl Station {
    pub fn get_message(&self) -> String {
        self.message.clone()
    }
}
impl Debug for Station {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f , "{}は、{}" , self.name , self.message)
    }
}

/// 入力された駅名を取得する
pub fn enter_station(message: &str) -> String {
    println!("{}" , message);
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).ok();
    name.trim().to_string()
}

