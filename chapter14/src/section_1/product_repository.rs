//!
//! 14章 PostgreSQL
//! 

use postgres::Transaction;
use postgres::types::Type;
use anyhow::{Result,Error};
use crate::section_1::repository::Repository;
use crate::section_1::entities::Product;
/// ## 14-4.CRUD操作
/// ### リスト14-8 Repositoryトレイトの実装
/// ### productテーブルをアクセスするRepository
pub struct ProductRepository<'a,'b> (pub &'a mut Transaction<'b>);
impl  ProductRepository<'_ , '_> {
    /// ## 14-5.CRUDの実装
    /// ### リスト14-15 avg_by_price()メソッドの実装
    //  ### 戻り値: f64:単価の平均
    pub fn avg_by_price<'a>(&mut self) -> Result<f64> {
        let row = self.0.query_one(
    "SELECT CAST(AVG(price) AS FLOAT) AS price_avg FROM product",&[])?;
        let result = row.get("price_avg");
        Ok(result)
    }
}
impl Repository<Product , i32 , u64> for ProductRepository<'_ , '_> {
    /// ## 14-5.CRUDの実装
    /// ### リスト14-9 select_all()メソッドの実装
    fn select_all(&mut self) -> Result<Vec<Product>> {
        let sql = "SELECT id,name,price,category_id FROM product"; // 利用するSQLステートメント
        let rows = self.0.query(sql , &[])?;// 問合せの実行
        let mut products = Vec::<Product>::new();
        // 問合せ結果を取得してベクタに格納する
        for row in rows.iter() {// 値の取り出しと格納
            // 各列の値を取り出してProductに格納してからVecに格納する
            products.push(Product::new(
                row.get("id") , 
                row.get("name") ,
             row.get("price") ,
              row.get("category_id") ,
               None));
        }
        Ok(products)
    }

    /// ## 14-5.CRUDの実装
    /// ### リスト14-10 select_by_id()メソッドの実装
    /// ### 引数 id:商品番号
    fn select_by_id(&mut self, id: i32) -> Result<Product> {
        // 利用するSQLステートメント
        let sql = "SELECT id,name,price,category_id FROM product WHERE id = $1";
        // プレースホルダの型設定
        let stmt = self.0.prepare_typed(sql ,&[Type::INT4])?;
        // 問合せの実行
        let result = self.0.query_opt(&stmt , &[&id])?;
        match result {
            Some(row) => Ok(Product::new(
                row.get("id"), row.get("name"),
            row.get("price"), row.get("category_id"), None)),
            None => Err(Error::msg(
            format!("指定された値:{}に該当するレコードが存在しません。", id)))
        }
    }

    /// ## 14-5.CRUDの実装
    /// ### リスト14-12 insert()メソッドの実装
    /// ### 引数 row:追加商品
    fn insert(&mut self, _row: Product) -> Result<u64> {
        // 利用するSQLステートメントとプレースホルダの型設定
        let stmt =  self.0.prepare_typed(
            "INSERT INTO product VALUES(nextval('product_seq'),$1,$2,$3)",&[Type::VARCHAR,Type::INT4,Type::INT4])?;
        // レコードの追加
        let result = self.0.execute(&stmt,&[_row.get_name(),_row.get_price(),_row.get_category_id()])?;
        Ok(result)
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
    fn select_all() -> Result<()> {
        let mut client = create_client(); // Clientの取得
        // トランザクションを開始する
        let mut transaction = TransactionUtil::start(&mut client , true)?;
        // ProductRepositoryを生成する
        let mut repository = ProductRepository(&mut transaction);
        let result = repository.select_all();
        match result {
            Ok(products) =>println!("{:?}" , products),
            Err(_) => assert!(false)
        }
        Ok(())
    }

    // select_by_id()メソッドのテスト
    #[test]
    fn select_by_id() -> Result<()> {
        let mut client = create_client(); // Clientの取得
        // トランザクションを開始する
        let mut transaction = TransactionUtil::start(&mut client , true)?;
        // ProductRepositoryを生成する
        let mut repository = ProductRepository(&mut transaction);
        // 存在する主キー値を利用する
        let result = repository.select_by_id(1);
        match result {
            Ok(product) =>println!("{:?}" , product.to_string()),
            Err(_) => assert!(false)
        }
        // 存在しない主キー値を指定する
        let result = repository.select_by_id(1000);
        match result {
            Ok(_) => assert!(false),
            Err(error) => println!("{:?}" , error.to_string())
        }
        Ok(())
    }

    // insert()メソッドのテスト
    #[test]
    fn insert() -> Result<()> {
        let mut client = create_client();
        let mut transaction = TransactionUtil::start(&mut client , false)?;
        let mut repository = ProductRepository(&mut transaction);
        let product = 
        Product::new(0 , "商品-ABC".to_string() , 200 , 1 , None);
        let result = repository.insert(product);
        match result {
            Ok(count) =>{
                let _ = TransactionUtil::commit(transaction)?; // トランザクションをコミットする
                assert_eq!(count ,1);
            },
            Err(_) => assert!(false)
        }
        Ok(())
    }    

    // avg_by_price()のテスト
    #[test]
    fn avg_by_price() -> Result<()> {
        let mut client = create_client();
        let mut transaction = TransactionUtil::start(&mut client , false)?;
        let mut repository = ProductRepository(&mut transaction);
        let result = repository.avg_by_price();
        match result {
            Ok(avg) =>println!("単価の平均 = {:?}" , avg as i64),
            Err(_) => assert!(false)
        }
        Ok(())
    }    
}