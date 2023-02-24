///
/// 9章 列挙型のサンプルコード
/// 

/// ## 9-3.構造体型バリアント
/// 形を表す列挙型
pub enum Shape{
    Rectangle   {height: f64 , width: f64}, // 四角形 height:高さ width:幅
    Triangle    {height: f64, bottom: f64}, // 三角形 height:高さ bottom:底辺
    Circle      {radius: f64}, // 円 radius:半径
    Trapezium   {upper: f64 , bottom: f64 , height: f64} // 台形 upper:上低 bottom:下底
}
impl Shape { // 形を表す列挙型の実装ブロック
    /// 図形毎の面積を計算して返す
    pub fn area(&self) -> f64 {
        match self {
            Self::Rectangle { height, width } => height * width,// 四角形の面積を求めて返す
            Self::Triangle { height, bottom } => height * bottom / 2.0, // 三角形の面積を求めて返す
            Self::Circle { radius } => radius * radius * std::f64::consts::PI, // 円の面積を求めて返す
            Self::Trapezium { upper, bottom, height } => (upper + bottom) * height / 2.0 // 台形の面積を求めて返す
        }
    }
}
impl ToString for Shape { // ToStringトレイトの実装
    fn to_string(&self) -> String {
        match self {
            Self::Rectangle {..} => "四角形です。",
            Self::Triangle {..} => "三角形です。",
            Self::Circle {..} => "円です。",
            Self::Trapezium {..} => "台形です。"
        }.to_string()
    }
}


/// ## 9-3.構造体型バリアント
/// ### リスト9-7 構造体型バリアントの利用
#[allow(dead_code)]
pub fn use_struct(){
    let rectangle   = Shape::Rectangle {height:10.0 , width:5.5};
    let triangle    = Shape::Triangle {height:10.0 , bottom:5.0};
    let circle      = Shape::Circle {radius:3.5};
    let trapezium   = Shape::Trapezium {bottom:5.0 , upper:3.0 , height:6.0 };
    println!("四角形の面積={}" , rectangle.area());
    println!("三角形の面積={}" , triangle.area());
    println!("円の面積={}" , circle.area());
    println!("台形の面積={}" ,trapezium.area());
}



