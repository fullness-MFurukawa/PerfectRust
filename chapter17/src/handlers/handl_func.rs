//!
//! 17章 サンプルコード
//! 


use actix_web::{get , Responder, web, HttpResponse};
use mime;
use log::info;

use sea_orm::{DatabaseConnection, TransactionTrait};
use serde::{Serialize,Deserialize};
///  ## 17-4 リクエストエクストラクタ
///  #### リスト17-10 パラメータを受け取る構造体
#[derive(Debug,Serialize,Deserialize)]
pub struct AddCalc{
    pub value1: Option<String> , // 値1
    pub value2: Option<String> , // 値2
    pub answer: Option<String>   // 結果
}
impl AddCalc {
    //　加算処理
    pub fn calc(&mut self){
        let func = |v: &String|{
            if v.eq("") {
                return 0;
            }else{
                v.parse::<i32>().unwrap()
            }
        };
        let value1 =  self.value1.as_ref().map_or_else(|| 0, |v|{
           func(v)
        });
        let value2 =  self.value2.as_ref().map_or_else(|| 0, |v|{
            func(v)
        });
        self.value1 = Some(value1.to_string());
        self.value2 = Some(value2.to_string());
        self.answer = Some((value1+value2).to_string());
    }
}
impl ToString for AddCalc{
    // 値の出力
    fn to_string(&self) -> String { 
        format!("{} + {} = {}" , 
        self.value1.as_ref().unwrap(),
        self.value2.as_ref().unwrap(),
        self.answer.as_ref().unwrap())
    }
}


/// ## 17-3.リクエストハンドラ
/// ### リスト17-5 アトリビュートの利用 
#[get("/calc/{value1}/{value2}")]
pub async fn calc_1(value: web::Path<(String , String)>) -> impl Responder {
    let val1 = value.0.parse::<i32>().unwrap();
    let val2 = value.1.parse::<i32>().unwrap();
    // 計算結果の文字列を生成
    let result = format!("{} = {} + {}", val1 + val2 , val1 , val2);
    // ステータス:200でHttpレスポンスを返す
    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(result) 
}

/// ## 17-4.リクエストエクストラクタ
/// ### リスト17-11 Queryを利用リクエストハンドラ
pub async fn calc_2(value: web::Query<AddCalc>) -> impl Responder {
    let mut value1 = value.into_inner();// エクストラクタから値を取り出す
    value1.calc();// 計算処理を実行する
    info!("{:?}" , value1.to_string()); // ログに出力する 
    // 計算結果をレスポンスを返す
    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(value1.to_string())
}

/// ## 17-4.リクエストエクストラクタ
/// ### リスト17-12 Fromを利用リクエストハンドラ
pub async fn calc_3(value: web::Form<AddCalc>) -> impl Responder {
    let mut value1 = value.into_inner();// エクストラクタから値を取り出す
    value1.calc();// 計算処理を実行する
    info!("{:?}" , value1.to_string()); // ログに出力する 
    // 計算結果をレスポンスを返す
    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(value1.to_string())
}

/// ## 17-4.リクエストエクストラクタ
/// ### リスト17-13 Jsonを利用リクエストハンドラ
pub async fn calc_4(value: web::Json<AddCalc>) -> impl Responder {
    let mut value1 = value.into_inner();// エクストラクタから値を取り出す
    value1.calc();// 計算処理を実行する
    info!("{:?}" , value1.to_string()); // ログに出力する 
    // 計算結果をレスポンスを返す
    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(value1.to_string())
}

/// ## 17-4.リクエストエクストラクタ
/// ### リスト17-16 Dataを利用リクエストハンドラ
pub async fn use_pool(pool: web::Data<DatabaseConnection>, value: web::Json<AddCalc>) -> impl Responder {
    
    let c = match pool.begin().await{
        Ok(t) => t,
        Err(err) => {
            info!("{:?}" , err.to_string());
            panic!()
        }
    };
    info!("トランザクション開始");
    let _ = c.rollback().await;
    info!("トランザクションをロールバック");
    
    
    let mut value1 = value.into_inner();// エクストラクタから値を取り出す
    value1.calc();// 計算処理を実行する
    info!("{:?}" , value1.to_string()); // ログに出力する 
    // 計算結果をレスポンスを返す
    HttpResponse::Ok().content_type(mime::TEXT_PLAIN).body(value1.to_string())
}

/// ## 17-6 レスポンス生成
/// ### リスト17-17 JSONボディの生成
pub async fn calc_5(value: web::Json<AddCalc>) -> impl Responder {
    let mut value1 = value.into_inner();// エクストラクタから値を取り出す
    value1.calc();// 計算処理を実行する
    info!("{:?}" , value1.to_string()); // ログに出力する 
    // 計算結果をレスポンスを返す
    HttpResponse::Ok().content_type(mime::APPLICATION_JSON).json(value1)
}