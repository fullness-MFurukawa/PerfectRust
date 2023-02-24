use std::sync::Arc;
use actix_web::{web , Responder};
use chapter18_lib::view_commons::dtos::UserDto;
use chapter18_lib::view_commons::jwt::{JwtEncoderImpl, JWT_COOKIE_KEY, ClaimsGenerator};
use tera::Tera;
use sea_orm::DatabaseConnection;
use chapter18_lib::domain::user::user::User;
use chapter18_lib::view_commons::validator::AppValidator;
use chapter18_lib::application::provider::ServiceProvider;
use chapter18_lib::view_commons::forms::LoginForm;
use chapter18_lib::view_commons::jwt::JwtEncoder;
use crate::Result;
use crate::handler::view_helper::UiHelper;
use super::web_jwt::WebClaims;

///
/// 認証 リクエストハンドラ
///
pub struct AuthenticateHandler;
impl AuthenticateHandler {
    // HTML Redirect PATH
    const VIEW_PATH:        &'static str = "pages/login/login.html";
    const MENU_REDIRECT:    &'static str = "/web_sample/menu";

    ///
    /// 認証
    /// ログイン画面要求
    ///
    pub async fn enter(tera: web::Data<Tera>) -> Result<impl Responder>  {
        Ok(UiHelper::create_resp(&tera , &tera::Context::new() ,Self::VIEW_PATH))
    }
    ///
    /// 認証
    /// ログイン認証
    ///
    pub async fn authenticate(
                            form: web::Form<LoginForm> ,
                            tera: web::Data<Tera> ,
                            pool: web::Data<Arc<DatabaseConnection>> ,
                            provider: web::Data<Arc<ServiceProvider>>) -> Result<impl Responder> {
         // 入力値の検証
         match form.validate_value() {
            Ok(_) => (),
            Err(error) => {
                let mut context = tera::Context::new();
                // 検証エラーをContextに格納
                context.insert("errors", &error.errors);
                return Ok(UiHelper::create_resp(&tera, &context, Self::VIEW_PATH));
            }
        };
        // 認証
        let user:User = form.0.into();
        match provider.authenticate_service.authenticate(&pool, &user).await{
            Ok(user) => {
                // JWTトークンを生成する
                let claims = WebClaims::generate(&UserDto::convert(user));
                let token = JwtEncoderImpl::encode(&claims);
                let cookie = actix_web::cookie::Cookie::build(
                    JWT_COOKIE_KEY, token).http_only(true).finish();
                Ok(UiHelper::found(Self::MENU_REDIRECT , Some(cookie)))
            },
            Err(error) => {
                //let err = WebAppError::AuthorizationError(error.to_string());
                let mut context = tera::Context::new();
                context.insert("error" ,&error.to_string());
                Ok(UiHelper::create_resp(&tera, &context, Self::VIEW_PATH))
            }
        }
    }
}