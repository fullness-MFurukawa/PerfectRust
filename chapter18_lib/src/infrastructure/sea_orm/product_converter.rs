
use crate::Result;
use crate::domain::converter::Converter;
use crate::domain::product::product::Product;
use crate::domain::category::category::Category;
use crate::infrastructure::sea_orm::model::product::Model;

///
/// ## SeaORM用 ProductとModelの相互変換
/// 
pub struct ProductConverterSeaOrm;
impl Converter<Product , Model> for ProductConverterSeaOrm {
    // Entityから他のモデルへ変換する
    fn from_entity(entity: &Product) -> Result<Model>{
        let product_id:i32 = entity.product_id.clone().try_into().unwrap();
        let product_name:String = entity.product_name.clone().try_into().unwrap();
        let product_price:i32 = entity.product_price.clone().try_into().unwrap();
        let category_id = match entity.category.clone() {
            Some(category) => category.category_id.clone().try_into().unwrap(),
            None => 0
        };
        let model = Model{id: product_id , name: Some(product_name) , 
            price: Some(product_price) , category_id: Some(category_id)};
        Ok(model)
    }
    // 他のモデルからEntityへ変換する
    fn from_model(model: &Model) -> Result<Product>{
        let category = Category::build(model.category_id.clone().unwrap(), String::from("dummy"))?;
        let product = Product::build(model.id , 
                      model.name.clone().unwrap() ,model.price.clone().unwrap() , Some(category))?;
        Ok(product)
    }
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn from_entity() {
        let category = Category::build(1, String::from("文房具")).unwrap();
        let product = 
        Product::build(1, String::from("商品-X"), 200, Some(category)).unwrap();
        let result = ProductConverterSeaOrm::from_entity(&product).unwrap();
        println!("{:?}" , result);
        assert!(true);
    }
    #[test]
    fn from_model() {
        let model = Model{id:1,name:Some(String::from("商品-Y")),price:Some(200),category_id:Some(1)};
        let result = ProductConverterSeaOrm::from_model(&model).unwrap();
        println!("{:?}" , result);
        assert!(true);
    }
}