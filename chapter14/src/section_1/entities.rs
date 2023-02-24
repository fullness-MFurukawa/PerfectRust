//!
//! 14章 PostgreSQL
//! 
use lombok::*;
/// ## 14-4.CRUD操作
/// ### リスト14-6 Entityの実装
/// ### 商品カテゴリ
#[derive(Getter,GetterMut,Setter,NoArgsConstructor,AllArgsConstructor,ToString,Clone)]
pub struct ProductCategory {
    id:             i32,    // 商品カテゴリ番号
    name:           String, // 商品カテゴリ名
    products:       Option<Vec<Product>> // 結合で取得した商品
}
/// ## 14-4.CRUD操作
/// ### リスト14-6 Entityの実装
/// ### 商品
#[derive(Getter,GetterMut,Setter,NoArgsConstructor,AllArgsConstructor,ToString,Clone)]
pub struct Product {
    id:             i32,    // 商品番号
    name:           String, // 商品名
    price:          i32,    // 単価
    category_id:    i32,    // 商品カテゴリ番号
    product_category:  Option<ProductCategory> // 結合で取得した商品カテゴリ
}