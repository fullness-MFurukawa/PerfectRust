use std::pin::Pin;
use std::future::Future;
use chrono::Duration;
use serde::{Serialize, Deserialize};
use actix_web::{FromRequest , HttpRequest};
use actix_web::dev::Payload;
use chapter18_lib::view_commons::dtos::UserDto;
use chapter18_lib::view_commons::jwt::ClaimsGenerator;
use chapter18_lib::view_commons::jwt::JwtDecoder;
use crate::handler::error::{ApiAppError,ApiErrorInfo};
///
/// 認証結果レスポンス用構造体
/// 
#[derive(Debug , Clone , Serialize , Deserialize)]
pub struct ClaimsResponse {
    pub state:  String ,
    pub claims: String
}
impl ClaimsResponse {
    pub fn new(_state: &str , _claims: &str) -> Self {
        Self{
            state:String::from(_state) ,
            claims:String::from(_claims)
        }
    }
}

///
/// クレーム(認証に必要な個人情報)
/// JWTトークンのPayload
/// 
#[derive(Debug , Serialize , Deserialize)]
pub struct ApiClaims {
    iat:        i64 ,      //  Token取得日時
    exp:        i64 ,      //  Tokenの有効期限
    sub:        String ,   //  リソースオーナーの識別子
    user_id:    String ,   //   ユーザーId(Uuid)
    user_name:  String     //   ユーザー名
}
impl ClaimsGenerator<UserDto> for ApiClaims {
    fn generate(user: &UserDto) -> Self {
        let now =  chrono::Utc::now();
        let _iat =  now.timestamp();
        // クレーム(Payload)の生成
        Self {
            iat: _iat , // 取得日時の設定
            exp: (now + Duration::minutes(5)).timestamp() , // 有効期限を5分に設定
            sub: String::from("Fullness.Inc,") , // オーナー識別子を設定
            user_id: user.user_id.clone() ,     // ユーザーidを設定
            user_name: user.user_name.clone()   // ユーザー名
        }
    }
}
///
/// リクエスト受信時の前処理
///
impl FromRequest for  ApiClaims {
    type Error = ApiAppError;
    type Future = Pin<Box<dyn Future<Output = anyhow::Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let request = req.clone();
        Box::pin(async move {
            let decoder = ApiJwtDecoder::default();
            let token = decoder.decode_header(&request)?;
            match decoder.decode_jwt_token(token.as_str()){
                Ok(token_data) => Ok(token_data.claims) ,
                Err(error) => Err(ApiAppError::NotAuthorizeError(
                    ApiErrorInfo::new("authorization error" , error.to_string().as_str())))
            }
        })
    }
}

///
/// API用Jwtトークンのデコード
///
#[derive(Default)]
pub struct ApiJwtDecoder;
impl JwtDecoder<ApiClaims , ApiAppError , HttpRequest> for ApiJwtDecoder{
    fn decode_header(&self , request: &HttpRequest) -> Result<String, ApiAppError> {
        // 認可情報ヘッダーの取得
        let header_value  = match request.headers().get("authorization"){
            Some(header) => header ,
            None => return Err(ApiAppError::NotAuthorizeError(
                ApiErrorInfo::new("authorization error" , "Authorization header not found.")))
        };
        // トークンの取得
        let token = header_value.to_str().unwrap();
        let mut split_token = token.split_whitespace();
        // スキーマの取得
        match split_token.next() {
            Some(schema_type) => {
                if schema_type != "Bearer" {
                    return Err(ApiAppError::NotAuthorizeError(
                        ApiErrorInfo::new("authorization error","An invalid schema type was specified.")));
                }
            },
            None => return Err(ApiAppError::NotAuthorizeError(
                ApiErrorInfo::new("authorization error","An invalid schema type was specified.")))
        };
        // JWTトークンの取得
        match split_token.next() {
            Some(jwt_token) => Ok(jwt_token.to_string()),
            None => Err(ApiAppError::NotAuthorizeError(
                ApiErrorInfo::new("authorization error","JWT token not found.")))
        }
    }
}
