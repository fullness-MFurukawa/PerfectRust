use std::fmt::{Display, Formatter};
use thiserror::Error;
use actix_web::{HttpResponse, ResponseError , http::StatusCode};
use log::{error,info};
use chapter18_lib::error::AppError;
use crate::handler::view_helper::UiHelper;
use crate::Result;
///
/// Webアプリケーションエラー型
///
#[derive(Debug , Error)]
pub enum WebAppError {
    InternalError(String) ,     // 内部エラー
    AuthorizationError(String)  // 利用認可エラー
}
impl WebAppError {
    // AppErrorからメッセージを取得する
    pub fn error_message(error: AppError) -> Result<String> {
        match error {
            // 内部エラーはWebAppErrorに変換して通知する
            AppError::InternalError(..) => Err(Self::InternalError(error.to_string())),
            AppError::AuthenticateError(msg) |
            AppError::RegisterError(msg) |
            AppError::SearchError(msg) => Ok(msg)
        }
    }
}
impl Display for WebAppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f , "{}" , self)
    }
}
// エラーのハンドリング
impl ResponseError for WebAppError {
    // ステータスコードの設定
    fn status_code(&self) -> StatusCode {
        match self{
            // データベース、ドメインルールエラー
            WebAppError::InternalError(..)  => StatusCode::INTERNAL_SERVER_ERROR ,
            // 認証処理エラー
            WebAppError::AuthorizationError(..) => StatusCode::UNAUTHORIZED
        }
    }
    fn error_response(&self) -> HttpResponse {
        let path = match self {
            WebAppError::InternalError(msg) => {
                error!("{:?}" , msg) ;
                "/web_sample/error" // エラー画面にリダイレクトする
            },
            WebAppError::AuthorizationError(msg) =>{
                info!("{:?}" , msg);
                "/web_sample/login" // ログイン認証へリダイレクトする
            }
        };
        UiHelper::found(path , None)
    }
}
