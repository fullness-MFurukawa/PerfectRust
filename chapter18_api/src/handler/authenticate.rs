use std::sync::Arc;
use actix_web::{HttpResponse, Responder, web};
use chapter18_lib::view_commons::dtos::UserDto;
use sea_orm::DatabaseConnection;
use mime::APPLICATION_JSON;
use chapter18_lib::view_commons::forms::LoginForm;
use chapter18_lib::view_commons::jwt::{ClaimsGenerator,JwtEncoder,JwtEncoderImpl};
use chapter18_lib::view_commons::validator::AppValidator;
use chapter18_lib::domain::user::user::User;
use chapter18_lib::application::provider::ServiceProvider;
use crate::Result;
use crate::handler::api_jwt::{ApiClaims,ClaimsResponse};
use crate::handler::error::ApiAppError;

///
/// 認証 リクエストハンドラ
///
pub struct AuthenticateHandler;
impl AuthenticateHandler {
    // ログイン認証
    pub async fn authenticate(form: web::Json<LoginForm>,
                              pool: web::Data<Arc<DatabaseConnection>>,
                              provider: web::Data<Arc<ServiceProvider>>) -> Result<impl Responder> {
        
        // 入力値の検証
        match form.validate_value() {
            Ok(_) => (),
            Err(error) =>
                return Ok(HttpResponse::BadRequest().content_type(APPLICATION_JSON).json(error.errors))
        };
        // 認証処理
        let user:User = form.0.into();
        match provider.authenticate_service.authenticate(&pool, &user).await{
            Ok(user) => {
                // JWTトークンの生成
                let claims = ApiClaims::generate(&UserDto::convert(user));
                let token = JwtEncoderImpl::encode(&claims);
                Ok(HttpResponse::Ok().content_type(APPLICATION_JSON).json(
                    ClaimsResponse ::new("authenticate success" , token.as_str())))
            },
            Err(error) => Err(ApiAppError::from(error))
        }
    }
}