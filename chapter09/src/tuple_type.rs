///
/// 9章 列挙型のサンプルコード
/// 


/// ## 9-2.タプル型バリアント
/// ### 季節を表す列挙型
#[derive(Debug)]
pub enum Season2<'a> {
    // u8:月数  Vec<&str>:月名
    Spring(u8 , Vec<&'a str>) ,
    Summer(u8 , Vec<&'a str>) ,
    Autumn(u8 , Vec<&'a str>) ,
    Winter(u8 , Vec<&'a str>) 
}
impl<'a> Season2<'a> { // 季節を表す列挙型の実装ブロック
    /// ## バリアントをフォーマット変換して返す
    /// ### 引数 選択されたバリアント　
    /// ### 戻り値 フィーマット変換した文字列
    pub fn format_valiant(&self) -> String {
        match self {
            Self::Spring(x , y) => format!("春:{}ヶ月 {:?}" , x , y),
            Self::Summer(x , y) => format!("夏:{}ヶ月 {:?}" , x , y),
            Self::Autumn(x , y) => format!("秋:{}ヶ月 {:?}" , x , y),
            Self::Winter(x , y) => format!("冬:{}ヶ月 {:?}" , x , y)
        }
    }
}

/// ## 9-2.タプル型バリアント
/// ### リスト9-5 タプル型バリアントの利用
#[allow(dead_code)]
pub fn use_tuple(){
    let spring = Season2::Spring(3 , vec!("3月","4月","5月"));
    let summer = Season2::Summer(3 , vec!("6月","7月","8月"));
    let autumn = Season2::Autumn(3 , vec!("9月","10月","11月"));
    let winter = Season2::Winter(3 , vec!("12月","1月","2月"));
    println!("{}" , spring.format_valiant());
    println!("{}" , summer.format_valiant());
    println!("{}" , autumn.format_valiant());
    println!("{}" , winter.format_valiant());
}


