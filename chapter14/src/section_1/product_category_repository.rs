//!
//! 14章 PostgreSQL
//! 

use postgres::Transaction;
use postgres::types::Type;
use anyhow::{Result,Error};
use crate::section_1::entities::{Product,ProductCategory};

/// ## 14-4.CRUD操作
/// ### リスト14-14 product_categpryテーブルをアクセスするRepository
pub struct ProductCategoryRepository<'a,'b> (pub &'a mut Transaction<'b>); 
impl ProductCategoryRepository<'_,'_>  {

    /// ## 14-5.CRUDの実装
    /// ### リスト14-14 select_by_id_join_product()メソッドの実装
    /// ### 引数 id:商品カテゴリ番号
    /// ### 指定された主キーに一致するレコードをproductと結合して取得する
    pub fn select_by_id_join_product(&mut self , id: i32) -> Result<ProductCategory> {
        // SQLステートメント
        let sql = format!("{}{}",
        "SELECT c.id AS c_id, c.name AS c_name, p.id , p.name , p.price , p.category_id ",
        "FROM product_category c JOIN product p ON c.id = p.category_id WHERE c.id = $1");

        // プレースホルダの型指定とプリペアード
        let stmt = self.0.prepare_typed(sql.as_str() ,&[Type::INT4])?;
        // 問合せの実行
        let rows = self.0.query(&stmt , &[&id])?;
        if rows.is_empty() {
            return Err(Error::msg(
            format!("指定された値:{}に該当するレコードが存在しません。",id)));
        }
        // ProductCategoryの値を生成する
        let mut product_category = ProductCategory::new(
        0 , String::from("") , None);
        // productテーブルの値を格納するVecを生成する
        let mut products = Vec::<Product>::new();
        for row in rows {
            // idフィールドの値が0?
            if product_category.get_id() == &0 {
                // product_categoryテーブルの値を格納する
                product_category.set_id(row.get("c_id"));
                product_category.set_name(row.get("c_name"));
            }
            // productテーブルの値をVecに格納する
            products.push(Product::new(row.get("id"), row.get("name"),
                row.get("price"), row.get("category_id"), None));
        }
        // productsフィールドにVecを格納する
        product_category.set_products(Some(products));
        Ok(product_category)
    }
}   
#[cfg(test)]
mod tests{
    use super::*;
    use postgres::Client;
    use crate::section_1::params::ConnectParams;
    use crate::section_1::connect::PostgresSampleClient;
    use crate::section_1::transaction::TransactionUtil;
    // Clinetの取得
    fn create_client() -> Client {
        // 接続情報の生成
        let params = ConnectParams::new(
            "localhost".to_owned() , 5432, "rust_sample".to_owned() ,
            "postgres".to_owned(), "postgres".to_owned());
        let client = match PostgresSampleClient::simple_connect(params.clone()){
            Ok(client) => client ,
            Err(error) => panic!("{:?}" , error.to_string())
        };
        client
    }

    #[test]
    fn select_by_id_join_product() -> Result<()> {
        let mut client = create_client(); // Clientの取得
        // トランザクションを開始する
        let mut transaction = TransactionUtil::start(&mut client , true)?;
        // ProductCategoryRepositoryを生成する
        let mut repository = ProductCategoryRepository(&mut transaction);
        let result = repository.select_by_id_join_product(1);
        match result {
            Ok(product_category) =>println!("{:?}" , product_category.to_string()),
            Err(_) => assert!(false)
        }
        Ok(())
    }
}