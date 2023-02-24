


use chapter18_lib::application::product::product_service_impl::ProductServiceImpl;
use chapter18_lib::domain::category::category::Category;
use chapter18_lib::domain::product::product::Product;
use chapter18_lib::domain::product::product_name::ProductName;
use crate::commons;

#[tokio::test]
async fn by_keyword(){
    let pool = commons::get_db().await;
    let service = ProductServiceImpl::new();
    match service.by_keyword(&pool, &ProductName::try_from(String::from("ボールペン")).unwrap()).await{
        Ok(products) => {
            for product in products.clone() {
                println!("{:?}" , product);
            }
            assert_eq!(6 , products.len());
        }
        Err(_) => panic!()
    }
    match service.by_keyword(&pool, &ProductName::try_from(String::from("ABC")).unwrap()).await{
        Ok(_) => panic!() , 
        Err(error) =>{
            println!("{:?}" , &error);
            assert_eq!(String::from("検索エラー:ABCに一致する商品は見つかりません。"),error.to_string());
        } 
    }
}

#[tokio::test]
async fn register() {
    let pool = commons::get_db().await;
    let service = ProductServiceImpl::new();
    let category = Category::build(1,String::from("dummy")).unwrap();
    let product = Product::build(
        0 , 
        String::from("商品-ABC") ,
        200 ,
        Some(category)
    ).unwrap();
    match service.register(&pool, &product).await{
        Ok(result) => {
            println!("{:?}" , result);
            assert!(true);
        },
        Err(_) => panic!()
    }
    match service.register(&pool, &product).await{
        Ok(_) => panic!(),
        Err(error) => assert_eq!(String::from("登録エラー:商品-ABCは既に登録済です。"),error.to_string())
    }
}