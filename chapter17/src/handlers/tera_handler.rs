//!
//! 17章 サンプルコード
//! 

use actix_web::{Responder, web, HttpResponse , error};
use serde::{Serialize,Deserialize};
use tera::Tera;

/// ## 17-6.Teraクレート
/// ### リスト17-21 リクエストパラメータ用構造
#[derive(Debug,Serialize,Deserialize)]
pub struct CalcForm{
    pub value1: Option<String> , // 値1
    pub value2: Option<String> , // 値2
    pub opt:    Option<String>   // 計算の種類
}impl CalcForm{
    pub fn calc(&self) -> anyhow::Result<i32> {
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
        let opt =  self.value2.as_ref().map_or_else(|| 1, |v|{
            func(v)
        });
        let result = match opt{
            1 => value1 + value1 ,
            2 => value1 - value2 ,
            3 => value1 * value2 ,
            4 => value1 / value2 ,
            5 => value1 % value2 ,
            _ => return Err(anyhow::Error::msg("Parameter Error."))
        };
        Ok(result)
    }
}

/// ## 17-6.Teraクレート
/// ### リスト17-20 GETリクエストへの応答
pub async fn calc_get(tera: web::Data<Tera>) -> impl Responder {
    // HTMLの取得
    let resp_body = tera.render(
        "pages/enter.html", &tera::Context::new())
        .map_err(|err| error::ErrorInternalServerError(err.to_string())).unwrap();
    // レスポンスの送信
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(resp_body)
}

/// ## 17-6.Teraクレート
/// ### リスト17-24 POSTリクエストへの応答
pub async fn calc_post(form: web::Form<CalcForm>,tera: web::Data<Tera>) -> impl Responder {
    let mut context = tera::Context::new(); // ミュータブルなContextを生成する
    let calc_form = form.into_inner(); // CalcFormを取得する
    // 計算した結果をContextに格納する
    match calc_form.calc(){
        Ok(result) =>  context.insert("result" , &result),
        Err(err) => context.insert("result", &err.to_string())
    }
    // HTMLコンテンツを取得する
    let resp_body = tera.render(
        "pages/result.html", &context)
        .map_err(|err| error::ErrorInternalServerError(err.to_string())).unwrap();
    HttpResponse::Ok().content_type(mime::TEXT_HTML).body(resp_body)
}
#[cfg(test)]
mod tests{

    use super::*;
    use actix_web::{App, web, test , Error};
    use actix_web::dev::ServiceResponse;
    use actix_web::http::StatusCode;
    use actix_web::web::resource;

    /// ##17-7.リクエストハンドラのテスト
    /// ### リスト17-25 テスト用サービスの生成 
    async fn test_service() -> impl actix_web::dev::Service<actix_http::Request, Response = ServiceResponse, Error = Error>{
        // Teraを生成する
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
        // テスト用サービスを生成する
        let test_service = test::init_service(
            App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(
                web::scope("/sample")
                .service(
                    resource("/calc_form")
                   .route(web::get().to(super::calc_get))
                   .route(web::post().to(super::calc_post)))
                )
        ).await;
        test_service
    }
    /// ##17-7.リクエストハンドラのテスト
    /// ### リスト17-26 GETリクエストのテスト
    #[actix_web::test]
    async fn calc_get()  {
        // テスト用Serviceの取得する
        let test_service = test_service().await;
        // GETリクエストを生成する
        let enter_request = test::TestRequest::get().uri("/sample/calc_form").to_request();
        // リクエストハンドラenter()を実行する
        let response = test::call_service(&test_service, enter_request).await;
        println!("{:?}" , response.headers());// ヘッダーを出力する
        println!("{:?}" , response.response().body());// ボディを出力する
        assert_eq!(response.status() , StatusCode::OK);// ステータスコードを評価する
    }
    /// ##17-7.リクエストハンドラのテスト
    /// ### リスト17-26 POSTリクエストのテスト
    #[actix_web::test]
    async fn test_answer() -> () {
        // テスト用Serviceの取得する
        let test_service = test_service().await;
        // 入力データを準備する
        let calc_form = CalcForm{value1:Some("100".to_owned()),
                    value2:Some("200".to_owned()),opt:Some("1".to_owned())}; 
         // CalcFormを格納したPostリクエストを生成する
        let answer_request = test::TestRequest::post().uri("/sample/calc_form").set_form(&calc_form).to_request();
        // リクエストハンドラanswer()を実行する
        let response = test::call_service(&test_service , answer_request).await;
        println!("{:?}" , response.headers());// ヘッダーを出力する
        println!("{:?}" , response.response().body());// ボディを出力する
        assert_eq!(response.status() , StatusCode::OK);// ステータスコードを評価する
    }


}