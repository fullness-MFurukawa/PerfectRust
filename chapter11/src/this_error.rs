///
/// 11章エラー サンプルコード
/// 

use std::str::FromStr;
use std::num::{ParseIntError, ParseFloatError};
use thiserror::Error;
use num_traits::NumOps;
/// ## 11-3.外部クレートの利用
/// ### リスト11-9 thiserrorを利用した独自エラー型
#[derive( Debug , Error)]
pub enum SampleError{
    #[error(transparent)]
    IntError(#[from] ParseIntError) ,   // ParseIntErrorを格納する
    #[error(transparent)]
    FloatError(#[from] ParseFloatError) // ParseFloatErrorを格納する
}
/* 
ParseIntErrorを受け取ってIntErrorバリアントにセットする
impl From<ParseIntError> for SampleError{
    fn from(value: ParseIntError) -> Self {
        Self::IntError(value.to_string())
    }
}
/// ParseFloatErrorを受け取ってFloatErrorバリアントにセットする
impl From<ParseFloatError> for SampleError {
    fn from(value: ParseFloatError) -> Self{
        Self::FloatError(value.to_string())
    }
}
*/

type Result<T> = anyhow::Result<T , SampleError>;
/// ## 11-3.外部クレートの利用
/// ### リスト11-9 #[from]アトリビュートの利用
/// ### 引数をジェネリクスで指定された型に変換する
/// ### 引数 value:変換対象文字列
/// ### 戻り値 Result<T , SampleError>
#[allow(dead_code)]
fn parse_03<T:NumOps + FromStr>(value: String) -> Result<T> 
where SampleError: From<<T as FromStr>::Err> {
    let result = value.parse::<T>().map_err(|error|
        SampleError::from(error) )?;
    Ok(result)
}
#[allow(dead_code)]
pub fn use_parse_03() {
    let result = parse_03::<i32>(String::from("123")).unwrap();
    println!("{:?}" , result);
    let result = parse_03::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}" , result.to_string());
    let result = parse_03::<f32>(String::from("123")).unwrap();
    println!("{:?}" , result);
    let result = parse_03::<f32>(String::from("ABC")).err().unwrap();
    println!("{:?}" , result.to_string());
}