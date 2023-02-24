use actix_web::{Responder, web};
use tera::Tera;
use crate::handler::web_jwt::WebClaims;
use crate::handler::view_helper::UiHelper;

pub struct MenuHandler;
impl  MenuHandler {
    pub const VIEW_PATH: &'static str =  "pages/menu/menu.html";
    pub async fn menu(_claims: WebClaims , tera: web::Data<Tera>) -> impl Responder  {
        UiHelper::create_resp(&tera,&tera::Context::new(),Self::VIEW_PATH)
    }
}
pub struct ErrorHandler;
impl ErrorHandler {
    pub const VIEW_PATH: &'static str =  "pages/error/error.html";
    pub async fn error(_claims: WebClaims , tera: web::Data<tera::Tera>) -> impl Responder  {
        UiHelper::create_resp(&tera,&tera::Context::new(),Self::VIEW_PATH)
    }
}