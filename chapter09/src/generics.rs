///
/// 9章 列挙型のサンプルコード
/// 

/// 9-4.ジェネリッス
/// 季節を表現する列挙型
#[allow(dead_code)]
#[derive(Debug , Clone)]
enum Season<T> {
    // u8:月数 T:期間
    Spring(u8 , T),
    Summer(u8 , T),
    Autumn(u8 , T),
    Winter(u8 , T)
}
impl<T> Season<T>  {
    /// ## 月の文字列を取得する
    /// ### トレイト境界 IntoIterator
    pub fn get_months(&self) -> &T where T:std::iter::IntoIterator  {
        match self {
            Self::Spring( _ , months) => months,
            Self::Summer( _ , months) => months,
            Self::Autumn( _ , months) => months,
            Self::Winter( _ , months) => months
        }
    }
}

/// ## 9-4.構造体型バリアント
/// ### リスト9-11 ジェネリクスを利用した列挙型の利用
#[allow(dead_code)]
pub fn use_generics(){
    use std::collections::LinkedList;
    // Vec<&str>で期間を指定する
    let spring = Season::Spring(3,vec!("3月","4月","5月"));
    println!("春:{:?}" , spring.get_months());
    // 配列で期間を指定する
    let summber = Season::Summer(3 , ["6月","7月","8月"]);
    println!("夏:{:?}" , summber.get_months());
    // Linkedで期間を指定する
    let autumn = Season::Autumn(3 , LinkedList::from(["9月","10月","11月"]));
    println!("秋:{:?}" , autumn.get_months());

}



