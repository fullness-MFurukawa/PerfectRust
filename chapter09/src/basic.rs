///
/// 9章 列挙型のサンプルコード
/// 

use std::fmt::Display;
use std::fmt::Formatter;
/// ## 9-1.列挙型の定義
/// ### 季節を表す列挙型
#[repr(u64)]
#[derive(Debug)]
#[allow(dead_code)]
pub enum Season {
    Spring  = 100,   //  春
    Summer  = 200,   //  夏
    Autumn  ,   //  秋
    Winter      //  冬
}
impl Display for Season { // Displayトレイトの実装
    /// ## 9-1.列挙型の定義
    /// ### リスト9-3 任意のフォーマットで値を出力する
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self{
            Self::Spring => write!(f , "Spring(春) : {}" , Self::Spring as u32),
            Self::Summer => write!(f , "Summer(夏) : {}" , Self::Summer as u32),
            Self::Autumn => write!(f , "Autumn(秋) : {}" , Self::Autumn as u32),
            Self::Winter => write!(f , "Winter(冬) : {}" , Self::Winter as u32)
        }
    }
}

/// ## 9-1.列挙型の定義
/// ### リスト9-2 列挙型の利用
#[allow(dead_code)]
pub fn use_season(){
    // バリアント夏を取得
    let summer = Season::Summer;
    // バリアント冬を取得
    let winter = Season::Winter;
    println!("{:?}" , summer);
    println!("{:?}" , winter);
    // 整数値に変換する
    let summer_num = Season::Summer as u8;
    let winter_num = Season::Winter as u8;
    println!("{:?}" , summer_num);
    println!("{:?}" , winter_num);
}

/// ## 9-1.列挙型の定義
/// ### リスト9-3 Displayトレイトの実装
#[allow(dead_code)]
pub fn use_fmt(){
    println!("{}" , Season::Summer);
    println!("{}" , Season::Autumn);
}

/// ## 9-1.列挙型の定義
/// ### リスト9-4 Displayトレイトの実装
#[allow(dead_code)]
pub fn use_repr(){
    println!("{}" , Season::Spring);
    println!("{}" , Season::Summer);
    println!("{}" , Season::Autumn);
    println!("{}" , Season::Winter);
}
