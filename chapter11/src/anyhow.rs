///
/// 11章エラー サンプルコード
/// 

use std::str::FromStr;
use num_traits::NumOps;
use std::num::{ParseIntError, ParseFloatError};
use thiserror::Error;
#[derive( Debug , Error)]
pub enum SampleError{
    #[error(transparent)]
    IntError(#[from] ParseIntError) ,   // ParseIntErrorを格納する
    #[error(transparent)]
    FloatError(#[from] ParseFloatError) // ParseFloatErrorを格納する
}
// Result型のエリアス
type SampleResult<T> = anyhow::Result<T , anyhow::Error>;
/// ## 11-3.外部クレートの利用
/// ### リスト11-10 anyhow::Errorの利用
/// ### 引数をジェネリクスで指定された型に変換する
/// ### 引数 value:変換対象文字列
/// ### 戻り値 SampleResult<T>
#[allow(dead_code)]
fn parse_04<T:NumOps + FromStr>(value: String) -> SampleResult<T> 
where SampleError: From<<T as FromStr>::Err> {
    let result = value.parse::<T>().map_err(|error|{
        // contextに格納する情報を生成する
        let context = format!("指定された値:{}は変換できませんでした" , value);
        // SampleErrorを生成する
        let err = SampleError::from(error);
        // SampleErrorとcontextを設定したanyhow::Errorを返す
        anyhow::Error::new(err).context(context)
    })?;
    Ok(result)
}
#[allow(dead_code)]
pub fn use_parse_04() {
    let result = parse_04::<i32>(String::from("ABC")).err().unwrap();
    println!("{:?}" , result);
    println!("{:?}" , result.source().unwrap());
}