use chapter18_lib::{application::category::category_service_impl::CategoryServiceImpl, domain::category::category_id::CategoryId};

use crate::commons;

#[tokio::test]
async fn categories() {
    let pool = commons::get_db().await;
    let service = CategoryServiceImpl::new();
    let categories = service.categories(&pool).await.unwrap();
    for category in &categories {
        println!("{:?}" , category);
    }
    assert_eq!(3 , categories.len())
}

#[tokio::test]
async fn categtory() {
    let pool = commons::get_db().await;
    let service = CategoryServiceImpl::new();
    match service.category(&pool, &CategoryId::try_from(1).unwrap()).await{
        Ok(result) => {
            println!("{:?}" , &result);
            let id:i32 = result.category_id.try_into().unwrap();
            let name:String = result.category_name.try_into().unwrap();
            assert_eq!(1 , id);
            assert_eq!(String::from("文房具") , name);
        },
        Err(_) => panic!()
    }
    match service.category(&pool, &CategoryId::try_from(100).unwrap()).await{
        Ok(_) => panic!(),
        Err(error) => {
            let message = error.to_string();
            println!("{:?}" , &message);
            assert_eq!(String::from("登録エラー:100に一致するカテゴリが見つかりません。"),message);
        }
    }
}