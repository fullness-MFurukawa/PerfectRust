use jsonwebtoken::{decode , DecodingKey, EncodingKey, TokenData, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub const JWT_SECRET_KEY:   &str = "app-secret";  // シークレットキー
pub const JWT_COOKIE_KEY:   &str = "authorize";   // Cookieキー

///
/// Claimsの生成
///
pub trait ClaimsGenerator<T>{
    fn generate(_: &T) -> Self;
}
//
/// JWTトークンエンコード
///
pub trait JwtEncoder {
    // JWTトークン生成
    fn encode<T:Serialize>(claims: &T) -> String;
}
pub struct JwtEncoderImpl;
impl JwtEncoder for JwtEncoderImpl{
    // JWTトークン生成
    fn encode<T:Serialize>(claims: &T) -> String {
        // Headerの生成
        let mut header = jsonwebtoken::Header::default();
        header.typ = Some(String::from("JWT")); // typeの設定
        header.alg = jsonwebtoken::Algorithm::HS256; // アルゴリズムの設定
        // Headerとクレームでトークンを生成
        jsonwebtoken::encode(&header , &claims ,
                             // シークレットキーでエンコード
                             &EncodingKey::from_secret(JWT_SECRET_KEY.as_ref())).unwrap()
    }
}
///
/// JTWトークンデコード
///
pub trait JwtDecoder<T:DeserializeOwned , E , R> {
    // ヘッダー(Cookie)のデコード
    fn decode_header(&self , request: &R) -> Result<String , E>;
    // JWTトークンのデコードと検証
    fn decode_jwt_token(&self , jwt: &str) -> Result<TokenData<T> , jsonwebtoken::errors::Error> {
        match decode::<T>(
            //　シークレットキーでデコード
            jwt, &DecodingKey::from_secret(JWT_SECRET_KEY.as_ref()),
            // トークンの検証とデコード
            &Validation::default()) {
            Ok(token) => Ok(token),
            Err(error) => Err(error)
        }
    }
}