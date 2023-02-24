use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{DatabaseTransaction,DatabaseConnection,TransactionTrait};
use crate::{Result,AppError};
use crate::domain::product::product_name::ProductName;
use crate::domain::product::repository::ProductRepository;
use crate::infrastructure::sea_orm::product_repository::ProductRepositorySeaOrm;
use crate::application::product::product_service::ProductService;
use crate::domain::product::product::Product;
///
/// ## 商品サービス Traitの実装
/// 
pub struct ProductServiceImpl{
    // ProductRepositoryの実装を保持するフィールド
    repository: Arc<dyn ProductRepository<Transaction=DatabaseTransaction>>
}
impl ProductServiceImpl {
     ///　インスタンス生成
     pub fn new() -> Arc<dyn ProductService<Database=DatabaseConnection>>{
        // ProductRepositoryの実装をフィールドに設定した結果を返す
        Arc::new(Self{ repository: ProductRepositorySeaOrm::new() })
    }    
}
#[async_trait]
impl ProductService for ProductServiceImpl{
    type Database = DatabaseConnection;
    /// ## キーワード検索
    async fn by_keyword(&self , db:&Self::Database , keyword: &ProductName) -> Result<Vec<Product>>{
        // トランザクションを開始する
        let tran = match db.begin().await{
            Ok(transaction) => transaction ,
            Err(error) => return Err(AppError::from(error))
        };
        match self.repository.select_by_name_like(&tran, &keyword).await{
            Ok(products) => {
                if products.is_empty() {
                    return Err(AppError::SearchError(format!("{}に一致する商品は見つかりません。",keyword.to_string())));
                }
                Ok(products)
            },
            Err(error) => return Err(AppError::from(error))
        }
    }
    /// ## 新商品の登録
    async fn register(&self , db:&Self::Database , product: &Product) -> Result<Product>{
        // トランザクションを開始する
        let tran = match db.begin().await{
            Ok(transaction) => transaction ,
            Err(error) => return Err(AppError::from(error))
        };
        // 登録商品の存在チェック
        if self.repository.exists(&tran, &product.product_name).await? {
            let name:String = product.product_name.clone().try_into().unwrap();
            return Err(AppError::RegisterError(format!("{}は既に登録済です。",name)));
        }
        // 商品の登録
        let result = self.repository.insert(&tran, &product).await?;
        tran.commit().await.unwrap(); // トランザクションのコミット
        Ok(result)
    }
}