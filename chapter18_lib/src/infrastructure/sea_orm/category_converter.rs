
use crate::Result;
use crate::domain::converter::Converter;
use crate::domain::category::category::Category;
use crate::infrastructure::sea_orm::model::product_category::Model;

///
/// ## SeaORM用 CategoryとModelの相互変換
/// 
pub struct CategoryConverterSeaOrm;
impl Converter<Category , Model> for CategoryConverterSeaOrm {
    // Entityから他のモデルへ変換する
    fn from_entity(entity: &Category) -> Result<Model>{
        let category_id:i32 = entity.category_id.clone().try_into().unwrap();
        let category_name:String = entity.category_name.clone().try_into().unwrap();
        let model = Model{id: category_id , name: Some(category_name)};
        Ok(model)
    }
    // 他のモデルからEntityへ変換する
    fn from_model(model: &Model) -> Result<Category>{
        let category = 
        Category::build(model.id , model.name.clone().unwrap().to_owned());
        category
    } 
}
#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn from_entity() {
        let category = Category::build(1 , String::from("文房具")).unwrap();
        let result = CategoryConverterSeaOrm::from_entity(&category).unwrap();
        println!("{:?}" , result);
        assert!(true);
    }
    #[test]
    fn from_model() {
        let model = Model{id: 1 , name: Some(String::from("文房具"))};
        let result = CategoryConverterSeaOrm::from_model(&model).unwrap();
        println!("{:?}" , result);
        assert!(true);
    }
}