use std::sync::Arc;
use mime::APPLICATION_JSON;
use actix_web::{Responder, web , HttpResponse};
use sea_orm::DatabaseConnection;
use chapter18_lib::application::provider::ServiceProvider;
use chapter18_lib::view_commons::forms::ProductRegisterForm;
use chapter18_lib::view_commons::validator::AppValidator;
use chapter18_lib::domain::product::product::Product;
use crate::Result;
use crate::handler::error::ApiAppError;
use crate::handler::api_jwt::ApiClaims;


///
/// 商品登録 リクエストハンドラ
///
pub struct ProductRegisterHandler;
impl ProductRegisterHandler {
    pub async fn register(_claims: ApiClaims ,
                          form: web::Json<ProductRegisterForm> ,
                          pool: web::Data<Arc<DatabaseConnection>>,
                          provider: web::Data<Arc<ServiceProvider>>) -> Result<impl Responder>{
        // 入力値の検証
        match form.validate_value() {
            Ok(_) => (),
            Err(error) =>
                return Ok(HttpResponse::BadRequest().content_type(APPLICATION_JSON).json(error.errors))
        };
        // 商品を永続化する
        let product:Product = form.0.into();
        let mut new_product = match provider.product_servcie.register(&pool, &product).await{
            Ok(new_product) =>  new_product ,
            Err(error) => return Err(ApiAppError::from(error))
        };
        match provider.category_service.category(&pool, &new_product.category.unwrap().category_id).await{
            Ok(category) => {
                new_product.category = Some(category);
                Ok(HttpResponse::Ok().content_type(APPLICATION_JSON).json(new_product))
            },
            Err(error) => return Err(ApiAppError::from(error))
        }
    }
}