use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::{ DatabaseTransaction, EntityTrait, QueryFilter, ColumnTrait, QueryOrder , ActiveModelTrait };
use sea_orm::ActiveValue::Set;
use crate::domain::converter::Converter;
use crate::{Result,AppError};
use crate::domain::product::product::Product;
use crate::domain::product::product_name::ProductName;
use crate::domain::product::repository::ProductRepository;
use crate::infrastructure::sea_orm::model::prelude;
use crate::infrastructure::sea_orm::model::product;
use super::category_converter::CategoryConverterSeaOrm;
use super::product_converter::ProductConverterSeaOrm;
///
/// ## productにアクセスするRepository
/// 
pub struct ProductRepositorySeaOrm;
impl ProductRepositorySeaOrm {
    ///　インスタンスを生成してUserRepositiryトレイト型で返す
    pub fn new() -> Arc<dyn ProductRepository<Transaction=DatabaseTransaction>> {
        Arc::new(Self{})
    }
}
#[async_trait]
impl ProductRepository for ProductRepositorySeaOrm {
    type Transaction = DatabaseTransaction;
    /// 商品キーワード検索する
    async fn select_by_name_like(&self , transaction: &Self::Transaction , keyword: &ProductName) -> Result<Vec<Product>>{
        // 指定されたキーワードで問合せし、商品番号でソートした結果を取得する
        let _keyword:String = keyword.clone().try_into().unwrap();
        match prelude::Product::find()
        .filter(product::Column::Name.contains(_keyword.as_str()))
        .find_also_related(prelude::ProductCategory)
        .order_by_asc(product::Column::Id)
        .all(transaction).await{
            Ok(models) => {
                let mut result:Vec<Product> = Vec::new();
                for model in models {
                    let category = CategoryConverterSeaOrm::from_model(&model.1.unwrap())?;
                    let mut product = ProductConverterSeaOrm::from_model(&model.0)?;
                    product.category = Some(category);
                    result.push(product);
                }
                Ok(result)
            },
            Err(error) => Err(AppError::from(error))
        }
    }
    /// 新しい商品を永続化する
    async fn insert(&self , transaction: &Self::Transaction , product: &Product) -> Result<Product>{
        let model = ProductConverterSeaOrm::from_entity(product).unwrap();
        // ActiveModelの生成
        let active_model = product::ActiveModel{
            name: Set(model.name),
            price: Set(model.price),
            category_id: Set(model.category_id),
            ..Default::default() // idの生成はシーケンスを利用する
        };
        match active_model.insert(transaction).await {
            Ok(new_model) => Ok(ProductConverterSeaOrm::from_model(&new_model).unwrap()) ,
            Err(error) => Err(AppError::from(error))
        }
    }
    /// 商品名の存在チェックする
    async fn exists(&self , transaction:&Self::Transaction , name: &ProductName) -> Result<bool>{
        let _name:String = name.clone().try_into()?;
        match prelude::Product::find().filter(product::Column::Name.eq(_name)).one(transaction).await{
            Ok(result) => Ok(result.is_some()) ,
            Err(error) => Err(AppError::from(error))
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use sea_orm::TransactionTrait;
    use crate::domain::category::category::Category;
    use crate::domain::category::category_id::CategoryId;
    use crate::domain::category::category_name::CategoryName;
    use crate::domain::product::product_price::ProductPrice;
    use crate::domain::product::product_id::ProductId;
    use crate::infrastructure::sea_orm::pool::PoolSeaOrm;
    use crate::infrastructure::pool::Pool;

    #[tokio::test]
    async fn select_by_name_like(){
        let pool = PoolSeaOrm::get().await.unwrap();
        let transaction = pool.begin().await.unwrap();
        let repository = ProductRepositorySeaOrm::new();
        let keyword = ProductName::try_from(String::from("マウス")).unwrap();
        match repository.select_by_name_like(&transaction, &keyword).await{
            Ok(results) =>{
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
        let keyword = ProductName::try_from(String::from("あいうえお")).unwrap();
        match repository.select_by_name_like(&transaction, &keyword).await{
            Ok(results) =>{
                if results.is_empty(){
                    assert!(true);
                }else{
                    assert!(false);
                }
            },
            Err(error) => {
                println!("{:?}" , error.to_string());
                assert!(false);
            }
        }
    }
    #[tokio::test]
    async fn insert(){
        let pool = PoolSeaOrm::get().await.unwrap();
        let transaction = pool.begin().await.unwrap();
        let repository = ProductRepositorySeaOrm::new();
        let product_id = ProductId::try_from(0).unwrap();
        let product_name = ProductName::try_from(String::from("商品-A")).unwrap();
        let product_price = ProductPrice::try_from(200).unwrap();
        let category_id = CategoryId::try_from(1).unwrap();
        let category_name = CategoryName::try_from(String::from("dummy")).unwrap();
        let category = Category::new(category_id, category_name).ok();
        let product = Product::new(product_id, product_name, product_price, category).unwrap();
        let result = repository.insert(&transaction, &product).await;
        transaction.commit().await.unwrap();
        match result {
            Ok(product) => {
                println!("{:?}" , product);
                assert!(true);
            },
            Err(error) =>{
                println!("{:?}" , error.to_string());
                assert!(false);
            }
        }
    }
    #[tokio::test]
    async fn exists(){
        let pool = PoolSeaOrm::get().await.unwrap();
        let transaction = pool.begin().await.unwrap();
        let repository = ProductRepositorySeaOrm::new();
        let product_name = ProductName::try_from(String::from("水性ボールペン(黒)")).unwrap();
        match repository.exists(&transaction, &product_name).await{
            Ok(result) => {
                if result {
                    assert!(true);
                }else{
                    assert!(false);
                }
            },
            Err(error) => {
                println!("{:?}" , error);
                assert!(false);
            }
        }
        let product_name = ProductName::try_from(String::from("水性ボールペン")).unwrap();
        match repository.exists(&transaction, &product_name).await{
            Ok(result) => {
                if result {
                    assert!(false);
                }else{
                    assert!(true);
                }
            },
            Err(error) => {
                println!("{:?}" , error);
                assert!(false);
            }
        }
    }

}