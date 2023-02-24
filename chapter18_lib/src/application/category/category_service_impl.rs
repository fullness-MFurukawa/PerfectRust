use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseTransaction,DatabaseConnection,TransactionTrait};
use crate::domain::category::category_id::CategoryId;
use crate::{Result,AppError};
use crate::domain::category::category::Category;
use crate::domain::category::repository::CategoryRepository;
use crate::infrastructure::sea_orm::category_repository::CategoryRepositorySeaOrm;
use super::category_service::CategoryService;

///
/// ## カテゴリサービス Traitの実装
/// 
pub struct CategoryServiceImpl{
    // CategoryRepositoryの実装を保持するフィールド
    repository: Arc<dyn CategoryRepository<Transaction=DatabaseTransaction>>
}
impl CategoryServiceImpl {
    ///　インスタンス生成
    pub fn new() -> Arc<dyn CategoryService<Database=DatabaseConnection>>{
        // CategoryRepositoryの実装をフィールドに設定した結果を返す
        Arc::new(Self{ repository: CategoryRepositorySeaOrm::new() })
    }    
}
#[async_trait]
impl CategoryService for CategoryServiceImpl{
    type Database = DatabaseConnection;
    /// ## 全件取得する
    async fn categories(&self , db:&Self::Database) -> Result<Vec<Category>>{
        // トランザクションを開始する
        let tran = match db.begin().await{
            Ok(transaction) => transaction ,
            Err(error) => return Err(AppError::from(error))
        };
        self.repository.select_all(&tran).await
    }
    /// ## 指定されたidのカテゴリを取得する
    async fn category(&self , db:&Self::Database , id: &CategoryId) -> Result<Category>{
        // トランザクションを開始する
        let tran = match db.begin().await{
            Ok(transaction) => transaction ,
            Err(error) => return Err(AppError::from(error))
        };
        match self.repository.select_by_id(&tran, &id).await?{
            Some(category) => Ok(category) ,
            None => Err(AppError::RegisterError(format!("{}に一致するカテゴリが見つかりません。",id.to_string())))
        }
    }
}