use actix_session::Session;
use actix_web::{HttpResponse , cookie::Cookie , http::header} ;
use tera::{Tera,Context};
use serde::{Serialize , de::DeserializeOwned};
use crate::{Result, WebAppError};
///
/// Webレスポンスの生成
/// 
pub struct UiHelper;
impl UiHelper {
    // HTMLレスポンス生成
    pub fn create_resp(tera: &Tera,context: &Context , path: &str) -> HttpResponse {
        let body = tera.render(path, context).unwrap();
        HttpResponse::Ok().content_type(mime::TEXT_HTML).body(body)
    }
    // リダイレクトレスポンスの生成
    pub fn found(path: &str , cookie: Option<Cookie>) -> HttpResponse {
        if cookie.is_some(){
            HttpResponse::Found().cookie(cookie.unwrap()).insert_header((header::LOCATION , path)).finish()
        }else {
            HttpResponse::Found().insert_header((header::LOCATION , path)).finish()
        }
    }
}
///
/// Session操作ヘルパ
///
pub struct SessionHelper;
impl SessionHelper {
    // 指定された値をSessionに登録する
    pub fn insert<T: Serialize>(session: &Session, key: &str , value: &T) -> Result<()> {
        match session.insert(key, value) {
            Ok(()) => Ok(()) ,
            Err(error) => Err(WebAppError::InternalError(error.to_string()))
        }
    }
    // Sessionに登録された値を削除する
    pub fn remove(session: &Session , key: &str) -> () {
        match session.remove(key) {
            Some(_) => () , None => ()
        }
    }
    // セッションから指定された値を取得する
    pub fn get<T: DeserializeOwned>(session: &Session , key: &str) -> Result<Option<T>>{
        match session.get(key){
            Ok(value) => Ok(value) ,
            Err(error) => Err(WebAppError::InternalError(error.to_string()))
        }
    }
}