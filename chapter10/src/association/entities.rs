///
/// 10章トレイト サンプルコード
/// 
use serde::Deserialize;

#[derive(Debug , Default ,Deserialize)]
/// 10-3.メソッドの実装
/// リスト10-9 商品を表す構造体
pub struct Product{
    pub id:         String, //  商品番号
    pub name:       String, //  商品名
    pub price:      u32     //  単価
}