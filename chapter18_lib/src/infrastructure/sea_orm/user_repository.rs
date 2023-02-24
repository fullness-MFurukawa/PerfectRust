
use std::sync::Arc;
use sea_orm::{DatabaseTransaction , EntityTrait , ColumnTrait , QueryFilter};
use async_trait::async_trait;
use crate::{Result,AppError};
use crate::domain::user::{repository::UserRepository,user_name::UserName,user::User};
use crate::domain::converter::Converter;
use super::user_converter::UserConverterSeaOrm;

/// ## 18-5 アプリケーションの構成
/// ### リスト18-28 UserRepositoryトレイトの実装
pub struct UserRepositorySeaOrm;
impl UserRepositorySeaOrm {
    ///　インスタンスを生成してUserRepositiryトレイト型で返す
    pub fn new() -> Arc<dyn UserRepository<Transaction=DatabaseTransaction>> {
        Arc::new(Self{})
    }
}
#[async_trait]
impl UserRepository for UserRepositorySeaOrm {
    type Transaction = DatabaseTransaction;
    /// 指定されたユーザー名で問合せする
    async fn select_by_name(&self , tran: &Self::Transaction, user_name: &UserName) -> Result<Option<User>>{
        let name:String = user_name.clone().try_into().unwrap();// 検索値を取得する
        match super::model::prelude::User::find()
        // UserNameに対応する列の値が存在したら、それを取得する
        .filter(super::model::user::Column::UserName.contains(name.as_str())).one(tran).await{
            Ok(option_model) => {
                match  option_model {
                    // データが取得できたらModelからUserに変換して返す
                    Some(model) => Ok(UserConverterSeaOrm::from_model(&model).ok()),
                    // 見つからない場合はNoneを返す
                    None => Ok(None)
                }
            },
            Err(error) => Err(AppError::from(error))
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    use sea_orm::TransactionTrait;
    use crate::infrastructure::sea_orm::pool::PoolSeaOrm;
    use crate::infrastructure::pool::Pool;

    #[tokio::test]
    async fn select_by_name() {
        let pool = PoolSeaOrm::get().await.unwrap();
        let transaction = pool.begin().await.unwrap();
        let user_name = UserName::try_from(String::from("user001")).unwrap();
        let repository = UserRepositorySeaOrm::new();
        let result = repository.select_by_name(&transaction, &user_name).await.unwrap();
        match result {
            Some(user) => {
                println!("{:?}" , user);
                assert!(true);
            }, 
            None => assert!(false)
        }
    }
}