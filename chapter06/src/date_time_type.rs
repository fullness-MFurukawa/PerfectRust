//!
//! 6章 ライブラリ型 サンプルプログラム
//! 

use chrono::prelude::*;     // chronoクレートのすべての機能を利用可能にする
use chrono_tz::Tz;          // タイムゾーンの機能を利用可能にする
use std::time::SystemTime;  // 標準ライブラリのSystemTimeを利用する

/// 6-2.日付・時間
/// リスト6-9 現在の日時を取得する
#[allow(dead_code)]
pub fn instantiate(){
    let now: DateTime<Utc> = Utc::now();// UTCタイムゾーンの日時を取得する
    println!("UTC日時 = {}" , &now);
    let now: DateTime<Local> = Local::now();// UTCタイムゾーンの日時を取得する
    println!("ローカル日時 = {}" , &now);
}
/// 6-2.日付・時間
/// リスト6-10 フォーマット変換
#[allow(dead_code)]
pub fn format(){
    // 日付をフォーマット変換する
    let now: DateTime<Utc> = Utc::now();
    let format_date= now.format("%Y年%m月%d日").to_string();
    println!("{:?}" , &format_date);
    // 日付時間をフォーマット変換する
    let now: DateTime<Local> = Local::now();
    let format_date_time = now.format("%Y年%m月%d日 %H時%M分%S秒").to_string();
    println!("{:?}" , &format_date_time);
}

/// 6-2.日付・時間
/// リスト6-11 文字列から日時に変換する
#[allow(dead_code)]
pub fn from_string(){
    // DateTime<T>変換メソッド
    let rfc2822_type = DateTime::parse_from_rfc2822("Fri, 14 Jan 2022 10:52:37 +0200");
    println!("{}", &rfc2822_type.unwrap());
    let rfc3339_type = DateTime::parse_from_rfc3339("2022-01-14T12:00:00-08:00");
    println!("{}", &rfc3339_type.unwrap());
    
    // nativeモジュールの変換機能
    let time_only = NaiveTime::parse_from_str("15:30:00", "%H:%M:%S");
    println!("{}", &time_only.unwrap());
    let date_only = NaiveDate::parse_from_str("2022年10月14日", "%Y年%m月%d日");
    println!("{}", &date_only.unwrap());
    let custom_format = NaiveDate::parse_from_str("10 2022 14", "%m %Y %d");
    println!("{}", &custom_format.unwrap());    
}

/// 6-2.日付・時間
/// リスト6-12 日時構成要素を取得する
#[allow(dead_code)]
pub fn get(){
    let now = Utc::now();
    // 年、月、日を取得する
    println!("y={},m={},d={}",&now.year(),&now.month(),&now.day());
    // 時、分、秒、ナノ秒を取得する
    println!("h={},m={},s={},n={}",&now.hour(),&now.minute(),&now.second(),&now.nanosecond());
    // 曜日を取得する
    let w = match &now.weekday(){
        Weekday::Mon => "月曜日",
        Weekday::Tue => "火曜日",
        Weekday::Wed => "水曜日",
        Weekday::Thu => "木曜日",
        Weekday::Fri => "金曜日",
        Weekday::Sat => "土曜日",
        Weekday::Sun => "日曜日"
    };
    println!("曜日={}" , &w);
}

/// 6-2.日付・時間
/// リスト6-13 日時構成要素を変更する
#[allow(dead_code)]
pub fn change(){
    let now = Utc::now();
    println!("取得した日時 = {}" , &now);
    let change = now.with_day(25);// 日を変更する
    println!("日を変更する = {}" , &change.unwrap());
    let change = now.with_month(10);// 月を変更する
    println!("月を変更する = {}" , &change.unwrap());
    let change = now.with_year(2023);// 年を変更する
    println!("年を変更する = {}" , &change.unwrap());
}

/// 6-2.日付・時間
/// リスト6-14 タイムゾーンを利用する
#[allow(dead_code)]
pub fn time_zone(){
    // Asia/Tokyoの日付・時間を取得する
    let tokyo: DateTime<Tz> = Local::now().with_timezone(&chrono_tz::Asia::Tokyo);
    println!("東京 = {}" , &tokyo); 
    // America/Chicagoの日付・時間を取得する
    let chicago: DateTime<Tz> = Local::now().with_timezone(&chrono_tz::America::Chicago);
    println!("シカゴ = {}" , &chicago);
    // タイムゾーンに依存した形式に変換する
    let tokyo_n = tokyo.naive_local();
    let chicago_n = chicago.naive_local();
    println!("{}" , &tokyo_n);
    println!("{}" , &chicago_n);   
    // 時間差を計算する
    let duration:chrono::Duration = tokyo_n - chicago_n;
    println!("時間数 = {}" , &duration.num_hours());
    println!("秒数 = {}" , &duration.num_seconds());
    println!("ナノ秒数 = {}" , &duration.num_nanoseconds().unwrap());
}

/// 6-2.日付・時間
/// リスト6-15 UNIXエポックを取得する
#[allow(dead_code)]
pub fn unix_epoch(){
    let x = Local::now().timestamp();
    println!("Localで取得:{}" , &x);
    let y = Utc::now().timestamp();
    println!("Utcで取得:{}" , &y);
    let z = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH);    
    println!("SystemTimeで取得:{}" , &z.unwrap().as_secs());
}



