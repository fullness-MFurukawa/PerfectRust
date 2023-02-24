//!
//! 14章 PostgreSQL
//!

use once_cell::sync::Lazy;
use yaml_rust::{Yaml, YamlLoader};
use std::sync::Mutex;
use anyhow::{Result , Error};
use std::env;
// SQLを保持するグローバル変数
static SQLS:Lazy<Mutex<Yaml>> = Lazy::new(||{
    init_sqls().unwrap_or_else(|error| panic!("{:?}" , error))
});
/// ## 14-8.YAMLファイルの利用
/// ### リスト14-27 resources/sql.ymlからSQLを取得する
/// ### 戻り値 Result<Mutex<Yaml>>:MutexでラップしたYAML
fn init_sqls() -> Result<Mutex<Yaml>> {
    let current = env::current_dir()
        .map(|path_buf| path_buf.join("resources\\sql.yml"))?;
    let str_data = std::fs::read_to_string( current.as_os_str())?;
    let values:Vec<Yaml> = YamlLoader::load_from_str(&str_data)?;
    let result = Mutex::new(values[0].clone());
    Ok(result)
}
/// ## 14-8.YAMLファイルの利用
/// ### リスト14-27 指定されらSQLを取得して返す
/// ### 引数 table_name:テーブル名
/// ### 引数 method_name:メソッド名
pub async fn get_sql(table_name: &str, method_name: &str) -> Result<String> {
    let yml = SQLS.lock().unwrap();
    let sqls = match yml[table_name].as_hash() {
        Some(sqls) => sqls,
        None => return Err(Error::msg(
                format!("テーブル名:{}が見つかりません。",table_name)))
    };
    let sql = match sqls.get(&Yaml::String(method_name.to_owned())){
        Some(sql) => sql.as_str().unwrap(),
        None => return Err(Error::msg(
                format!("メソッド名:{}が見つかりません。",method_name)))
    };
    Ok(sql.to_owned())
}
#[cfg(test)]
mod test{
    use super::*;
    use anyhow::Result;
    #[tokio::test]
    async fn get_sql_test() -> Result<()> {
        let sql = get_sql("product", "select_all").await?;
        println!("{:?}" , sql);
        let err = get_sql("product1", "select_by_id").await.err();
        println!("{:?}" , err.unwrap());
        let err = get_sql("product", "select_by_id1").await.err();
        println!("{:?}" , err.unwrap());
        Ok(())
    }
}
