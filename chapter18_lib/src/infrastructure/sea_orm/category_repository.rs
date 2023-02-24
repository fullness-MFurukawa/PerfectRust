

use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseTransaction,EntityTrait};
use crate::domain::converter::Converter;
use crate::{Result,AppError};
use crate::domain::category::category_id::CategoryId;
use crate::domain::category::category::Category;
use crate::domain::category::repository::CategoryRepository;
use crate::infrastructure::sea_orm::category_converter::CategoryConverterSeaOrm;
use crate::infrastructure::sea_orm::model::prelude::ProductCategory;

///
/// ## product_categoryにアクセスするRepository
/// 
pub struct CategoryRepositorySeaOrm;
impl CategoryRepositorySeaOrm {
    ///　インスタンスを生成してUserRepositiryトレイト型で返す
    pub fn new() -> Arc<dyn CategoryRepository<Transaction=DatabaseTransaction>> {
        Arc::new(Self{})
    }
}
#[async_trait]
impl CategoryRepository for CategoryRepositorySeaOrm {
    type Transaction = DatabaseTransaction;
    ///　すべてのカテゴリを取得する
    async fn select_all(&self , transaction: &Self::Transaction) -> Result<Vec<Category>>{
        match ProductCategory::find().all(transaction).await {
            Ok(models) =>{
                let mut results:Vec<Category> = Vec::new(); 
                for model in models {
                    let category = CategoryConverterSeaOrm::from_model(&model)?;
                    results.push(category);
                }
                Ok(results)
            },
            Err(error) => Err(AppError::from(error))
        }
    }
    ///　指定された識別子でカテゴリを取得する
    async fn select_by_id(&self , transaction: &Self::Transaction , id: &CategoryId) -> Result<Option<Category>>{
        let category_id:i32 = id.clone().try_into().unwrap();
        match ProductCategory::find_by_id(category_id).one(transaction).await {
            Ok(option_model) => {
                match option_model{
                    Some(model) => Ok(CategoryConverterSeaOrm::from_model(&model).ok()) ,
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
    async fn select_all() {
        let pool = PoolSeaOrm::get().await.unwrap();
        let transaction = pool.begin().await.unwrap();
        let repository = CategoryRepositorySeaOrm::new();
        match repository.select_all(&transaction).await {
            Ok(results) => {
                for result in results {
                    println!("{:?}" , result);
                }
                assert!(true);
            },
            Err(error) => {
                println!("{:?}" , error.to_string());
                assert!(false);
            }
        }
    }
    #[tokio::test]
    async fn select_by_id() {
        let pool = PoolSeaOrm::get().await.unwrap();
        let transaction = pool.begin().await.unwrap();
        let repository = CategoryRepositorySeaOrm::new();
        let category_id = CategoryId::try_from(1).unwrap();
        match repository.select_by_id(&transaction, &category_id).await {
            Ok(result) => {
                match result {
                    Some(r) => {
                        println!("{:?}" , r);
                        assert!(true)
                    },
                    None => assert!(false)
                }
            },
            Err(error) => {
                println!("{:?}" , error.to_string());
                assert!(false);
            }
        }
        let category_id = CategoryId::try_from(100).unwrap();
        match repository.select_by_id(&transaction, &category_id).await {
            Ok(result) => {
                match result {
                    Some(_) => assert!(false),
                    None => assert!(true)
                }
            },
            Err(error) => {
                println!("{:?}" , error.to_string());
                assert!(false);
            }
        }
    }

}