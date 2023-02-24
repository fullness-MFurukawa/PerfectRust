///
/// 10章トレイト サンプルコード
/// 

use anyhow::Result;

/* 
 ## 10-1.トレイトの基本
 ### リスト10-2 todo!()マクロ
trait Calculator {
    /// 計算処理メソッド 
    fn calc(&self) -> Result<u64>;
}
*/

/// ## 10-1.トレイトの基本
/// ### リスト10-2 todo!()マクロ
trait Calculator {
    /// 計算処理メソッド 
    fn calc(&self) -> Result<u64>{
        todo!("まだ実装されていません")
    }
}

/// ## 10-1.トレイトの基本
/// ### リスト10-3 トレイトの実装
/// ### 四角形を表現した構造体
pub struct Rectangle {
    width:  u64 ,   // 幅を表すフィールド
    height: u64     // 高さを表すフィールド
}
/// 構造体にCalculatorトレイトを実装
impl Calculator for Rectangle {
    /// 面積を計算させる
    fn calc(&self) -> Result<u64>{
        Ok(self.height*self.width)
    }
}

/// ## 10-1.トレイトの基本
/// ### リスト10-4 トレイトのメソッドを実行する
#[allow(dead_code)]
pub fn use_rectangle(){
    let r = Rectangle{width:100,height:50}; // インスタンスを生成
    let result = r.calc(); //　トレイトのメソッドを実行
    println!("面積 = {}" , result.unwrap());
}

