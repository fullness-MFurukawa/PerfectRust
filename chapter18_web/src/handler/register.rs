use std::sync::Arc;
use actix_session::Session;
use actix_web::{Responder , web};
use chapter18_lib::domain::product::product::Product;
use sea_orm::DatabaseConnection;
use tera::Tera;
use chapter18_lib::application::provider::ServiceProvider;
use chapter18_lib::view_commons::dtos::{CategoryDto, ProductDto};
use chapter18_lib::view_commons::forms::ProductRegisterForm;
use chapter18_lib::view_commons::validator::AppValidator;
use crate::Result;
use crate::handler::view_helper::UiHelper;
use crate::handler::view_helper::SessionHelper;
use crate::handler::error::WebAppError;
use crate::handler::web_jwt::WebClaims;

///
/// 商品登録 リクエストハンドラ
///
pub struct RegisterHandler;
impl RegisterHandler{
    // HTML Redirect PATH
    const ENTER_PATH: &'static  str = "pages/register/enter.html";
    const FINISH_PATH: &'static str = "pages/register/finish.html";
    const ENTER_REDIRECT: &'static  str = "/web_sample/register/product";
    const REGISTER_REDIRECT: &'static  str = "/web_sample/register/product/register";
    const FINISH_REDIRECT: &'static str = "/web_sample/register/product/finish";
    ///
    /// 商品登録　
    /// 商品入力画面要求への応答
    ///
    pub async fn enter(_claims: WebClaims,
                       session: Session ,
                       tera: web::Data<Tera>,
                       pool: web::Data<Arc<DatabaseConnection>>,
                       provider: web::Data<Arc<ServiceProvider>>) -> Result<impl Responder> {
        // セッションから商品カテゴリを取得する
        let session_categories = SessionHelper::get::<Vec<CategoryDto>>(&session,"categories")?;
        let categories = match session_categories {
            Some(categories) => categories ,
            None => {
                // 永続化層から商品カテゴリを取得する
                let categories = match provider.category_service.categories(&pool).await {
                    Ok(categories) => CategoryDto::converts(categories),
                    Err(error) => return Err(WebAppError::InternalError(error.to_string()))
                };
                // セッションに商品カテゴリを登録
                SessionHelper::insert::<Vec<CategoryDto>>(&session , "categories" , &categories)?;
                categories
            }
        };
        // TeraのContextに商品カテゴリを登録する
        let mut context = tera::Context::new();
        context.insert("categories" , &categories);
        Ok(UiHelper::create_resp(&tera , &context ,Self::ENTER_PATH))
    }  
    ///
    /// 商品登録　
    /// 入力値検証
    ///
    pub async fn confirm(_claims: WebClaims,
                         session: Session ,
                         form: web::Form<ProductRegisterForm> ,
                         tera: web::Data<tera::Tera>) -> Result<impl Responder> {
        // 入力値の検証
        match form.validate_value() {
            Err(error) => {
                let mut context = tera::Context::new();
                // 検証エラー、Form、カテゴリをContextに格納
                context.insert("form" , &form);
                // セッションからカテゴリを取得
                let categories = SessionHelper::get::<Vec<CategoryDto>>(&session,"categories")?.unwrap();
                context.insert("categories" , &categories);
                context.insert("errors", &error.errors);
                //　入力画面に遷移する
                Ok(UiHelper::create_resp(&tera, &context, Self::ENTER_PATH))
            }, 
            // 登録結果へリダイレクト
            Ok(_) => {
                // FormをSessionに登録する
                SessionHelper::insert(&session, "register_form", &form.into_inner())?;
                Ok(UiHelper::found(Self::REGISTER_REDIRECT , None))
            }
        }
    }
    ///
    /// 商品登録　
    /// 登録処理
    ///
    pub async fn complete(_claims: WebClaims,
                          session: Session ,
                          tera: web::Data<tera::Tera>,
                          pool: web::Data<Arc<DatabaseConnection>> ,
                          provider: web::Data<Arc<ServiceProvider>>) -> Result<impl Responder> {
        // セッションからFormを取得
        let form = SessionHelper::get::<ProductRegisterForm>(&session,"register_form")?.unwrap();
        // 商品を永続化する
        let product:Product = form.clone().into();
        match provider.product_servcie.register(&pool, &product).await{
            Ok(mut new_product) => {
                let category_id = new_product.category.unwrap().category_id;
                let category = provider.category_service.category(&pool, &category_id).await.unwrap();
                new_product.category = Some(category);
                let product_dto = ProductDto::convert(new_product);
                // 登録結果をSessionに格納する
                SessionHelper::insert::<ProductDto>(&session , "new_product" , &product_dto)?;
                // 登録結果へリダイレクト
                Ok(UiHelper::found(Self::FINISH_REDIRECT , None))
            },
            Err(error) => {
                //　登録済みの場合、入力画面に戻る
                let mut context = tera::Context::new();
                // セッションからカテゴリを取得
                let categories = SessionHelper::get::<Vec<CategoryDto>>(&session,"categories")?.unwrap();
                context.insert("categories" , &categories);
                context.insert("exists" , &WebAppError::error_message(error)?);
                context.insert("form" , &form);
                Ok(UiHelper::create_resp(&tera, &context , Self::ENTER_PATH))
            }
        }
    }
    ///
    /// 商品登録　登録結果の出力
    ///
    pub async fn finish(_claims: WebClaims,
                        session: Session ,
                        tera: web::Data<tera::Tera>) -> Result<impl Responder> {
        //  セッションから登録された商品情報を取得する
        match SessionHelper::get::<ProductDto>(&session, "new_product")?{
            Some(new_product) => {
                println!("{:?}" , &new_product);
                // TeraのContextに登録する
                let mut context = tera::Context::new();
                context.insert("new_product" , &new_product.clone());

                // 商品情報とFormをセッションから削除する
                SessionHelper::remove(&session , "new_product");
                SessionHelper::remove(&session, "register_form");
                // 完了画面を返す
                Ok(UiHelper::create_resp(&tera , &context,Self::FINISH_PATH))
            },
            None =>
                // 入力画面にリダイレクトする
                Ok(UiHelper::found(Self::ENTER_REDIRECT, None))
        }
    }
}