
use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection,DatabaseTransaction,TransactionTrait};
use super::user_service::UserService;
use crate::infrastructure::sea_orm::user_repository::UserRepositorySeaOrm;
use crate::{Result,AppError};
use crate::domain::user::user::User;
use crate::domain::user::repository::UserRepository;
/// ## 18-5 アプリケーションの構成
/// ### リスト18-30 UserServiceトレイトの実装
pub struct UserServiceImpl{
    // UserRepositoryの実装を保持するフィールド
    repository: Arc<dyn UserRepository<Transaction=DatabaseTransaction>>
}
impl UserServiceImpl {
    ///　インスタンス生成
    pub fn new() -> Arc<dyn UserService<Database=DatabaseConnection>>{
        // UserRepositoryの実装をフィールドに設定した結果を返す
        Arc::new(Self{ repository: UserRepositorySeaOrm::new() })
    }    
}
#[async_trait]
impl UserService for UserServiceImpl {
    type Database = DatabaseConnection;
    /// ユーザーを認証する
    async fn authenticate(&self , db:&Self::Database , user: &User) -> Result<User>{
        // トランザクションを開始する
        let tran = match db.begin().await{
            Ok(transaction) => transaction ,
            Err(error) => return Err(AppError::from(error))
        };
        // ユーザー名で問合せする
        let opt_user = self.repository.select_by_name(&tran , &user.user_name).await?;
        match opt_user {
            Some(get_user) => {
                // パスワードを比較する
                if user.password.eq(&get_user.password){
                    Ok(get_user.clone()) // パスワードが同じなら、Userのクローンを返す
                }else{
                    Err(AppError::AuthenticateError(String::from("パスワードが異なります。")))
                }
            },
            None => Err(AppError::AuthenticateError(String::from("存在しないユーザー名です。")))
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::sea_orm::pool::PoolSeaOrm;
    use crate::infrastructure::pool::Pool;

    #[tokio::test]
    async fn authenticate_ok() {
        let pool = PoolSeaOrm::get().await.unwrap();
        let service = UserServiceImpl::new();
        let user =  User::build(
            String::from("5772a800-fef1-40bf-888b-68fddd29d881"),
            String::from("user001"),
            String::from("a034408b78dfee92cdbfc6e5247cf0ece119f30e6ba7653f4b7a6f2f384f92a3c7cd4a0ec914ae3fb1ea93684b46f8ff2644ec0198d67be2fd2cbf68587f07b8"),
        String::from("yamada@sample.com"));
        let result = service.authenticate(&pool, &user).await;
        println!("{:?}" , result);
        assert!(true);
    }
    
    #[tokio::test]
    async fn authenticate_non_user() {
        let pool = PoolSeaOrm::get().await.unwrap();
        let service = UserServiceImpl::new();
        let user =  User::build(
            String::from("5772a800-fef1-40bf-888b-68fddd29d881"),
            String::from("user005"),
            String::from("a034408b78dfee92cdbfc6e5247cf0ece119f30e6ba7653f4b7a6f2f384f92a3c7cd4a0ec914ae3fb1ea93684b46f8ff2644ec0198d67be2fd2cbf68587f07b8"),
        String::from("yamada@sample.com"));
        let result = service.authenticate(&pool, &user).await;
        println!("{:?}" , result);
        assert!(true);
    }

    #[tokio::test]
    async fn authenticate_password() {
        let pool = PoolSeaOrm::get().await.unwrap();
        let service = UserServiceImpl::new();
        let user =  User::build(
            String::from("5772a800-fef1-40bf-888b-68fddd29d881"),
            String::from("user001"),
            String::from("a034408b78dfee92cdbfc6e5247cf0ece119f30e6ba7653f4b7a6f2f384f92a3c7cd4a0ec914ae3fb1ea93684b46f8ff2644ec0198d67be2fd2cbf68587f07b9"),
        String::from("yamada@sample.com"));
        let result = service.authenticate(&pool, &user).await;
        println!("{:?}" , result);
        assert!(true);
    }
}