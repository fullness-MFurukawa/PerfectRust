
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use super::user::{user_service::UserService, user_service_impl::UserServiceImpl};
use super::product::{product_service::ProductService,product_service_impl::ProductServiceImpl};
use super::category::{category_service::CategoryService,category_service_impl::CategoryServiceImpl};
/// ## 18-5 アプリケーションの構成
/// ### リスト18-31 ServiceProvider
pub trait Provider: Send + Sync {}
#[derive(Clone)]
pub struct ServiceProvider{
     // ユーザー認証サービス
     pub authenticate_service: Arc<dyn UserService<Database=DatabaseConnection>>,
     // カテゴリサービス
     pub category_service: Arc<dyn CategoryService<Database=DatabaseConnection>>,
     // 商品サービス
     pub product_servcie: Arc<dyn ProductService<Database=DatabaseConnection>>
}
impl ServiceProvider{
    pub fn new() -> Arc<Self>{
        Arc::new(
            Self{
                // ユーザー認証サービスの生成
                authenticate_service: UserServiceImpl::new(),
                // カテゴリサービスの生成
                category_service: CategoryServiceImpl::new(),
                // 商品サービスの生成
                product_servcie: ProductServiceImpl::new() 
            }
        )
    }
}
impl Provider for ServiceProvider{}

#[cfg(test)]
mod tests{
    use super::*;
    use crate::infrastructure::sea_orm::pool::PoolSeaOrm;
    use crate::infrastructure::pool::Pool;
    use crate::domain::user::user::User;
    #[tokio::test]
    async fn new_test(){
        let provider = ServiceProvider::new();
        let pool = PoolSeaOrm::get().await.unwrap();
        let user =  User::build(
            String::from("5772a800-fef1-40bf-888b-68fddd29d881"),
            String::from("user001"),
            String::from("a034408b78dfee92cdbfc6e5247cf0ece119f30e6ba7653f4b7a6f2f384f92a3c7cd4a0ec914ae3fb1ea93684b46f8ff2644ec0198d67be2fd2cbf68587f07b8"),
            String::from("yamada@sample.com"));
        let result = provider.authenticate_service.authenticate(&pool, &user).await;
        println!("{:?}" , result);
        assert!(true);
    }
    
}
