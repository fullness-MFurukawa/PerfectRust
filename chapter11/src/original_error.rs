///
/// 11章エラー サンプルコード
/// 


use std::fmt::{Display , Formatter};
use std::num::{ParseIntError, ParseFloatError};
use std::error::Error;
/// ## 11-2.独自エラー型
/// ### リスト11-4 関数で利用する独自エラー型
#[derive(Debug)]
pub enum SampleError{
    IntError(ParseIntError) ,   // 整数変換エラー
    FloatError(ParseFloatError) // 浮動小数点変換エラー
}
impl Error for SampleError {
}
impl Display for SampleError{ 
    /// to_string()メソッドで返されるフォーマット変換文字列の生成
    fn fmt(&self ,f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SampleError::IntError(err) => write!(f , "整数変換エラー:{}" , err),
            SampleError::FloatError(err) => write!(f , "浮動小数点変換エラー:{}" , err)
        }
    }
}
/// ParseIntErrorを受け取ってIntErrorバリアントにセットする
impl From<ParseIntError> for SampleError{
    fn from(value: ParseIntError) -> Self {
        Self::IntError(value)
    }
}
/// ParseFloatErrorを受け取ってFloatErrorバリアントにセットする
impl From<ParseFloatError> for SampleError {
    fn from(value: ParseFloatError) -> Self{
        Self::FloatError(value)
    }
}

use std::str::FromStr;
use num_traits::NumOps;
/// ## 11-2.独自エラー型
/// ### リスト11-6 独自エラー型を利用する関数
/// ### 引数をジェネリクスで指定された型に変換する
/// ### 引数 value:変換対象文字列
/// ### 戻り値 Result<T , SampleError>
#[allow(dead_code)]
fn parse_02<T:NumOps + FromStr>(value: String) -> Result<T , SampleError> 
where SampleError: From<<T as FromStr>::Err> {
    let result = value.parse::<T>().map_err(|error|
        SampleError::from(error) )?;
    Ok(result)
}

/// ## 11-2.独自エラー型
/// ### リスト11-6 独自エラー型を利用する関数
/// 
#[allow(dead_code)]
pub fn use_parse_02() {
    let result = parse_02::<i32>(String::from("123")).unwrap();
    println!("{:?}" , result);
    let result = parse_02::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}" , result.to_string());
    let result = parse_02::<f32>(String::from("123")).unwrap();
    println!("{:?}" , result);
    let result = parse_02::<f32>(String::from("ABC")).err().unwrap();
    println!("{:?}" , result.to_string());
}
