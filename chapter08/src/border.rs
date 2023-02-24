
use std::ops::Add;
use std::marker::Copy;
/// 8-7.トレイト境界
/// ## 顧客を表す構造体
#[allow(dead_code)]
#[derive(Debug,Clone)]
struct Customer<T> {
    id: T ,    //  顧客番号を表すフィールド
    name: String ,  //  氏名を表すフィールド
    address: String,//  住所を表すフィールド
    email: String   //  メールアドレスを表すフィールド
}
impl<T> Customer<T> where T:Copy + Add { // Customer構造体の実装ブロック
    /// ## 8-6.ジェネリクス
    /// ### リスト8-18 idの値を変更するメソッド
    #[allow(dead_code)]
    fn change_id(&mut self , value: T) {
        self.id = value;
    }
} 