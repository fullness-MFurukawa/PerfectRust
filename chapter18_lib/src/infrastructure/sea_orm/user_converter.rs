///
/// 18章 外部クレート活用
/// 

use crate::Result;
use crate::domain::converter::Converter;
use crate::domain::user::user::User;
use crate::infrastructure::sea_orm::model::user::Model;
/// ## 18-5 アプリケーションの構成
/// ### リスト18-27 UserとModelの相互変換
pub struct UserConverterSeaOrm;
impl Converter<User , Model> for UserConverterSeaOrm {
    // Entityから他のモデルへ変換する
    fn from_entity(entity: &User) -> Result<Model>{
        let id = 0;
        let user_id   = entity.user_id.clone().try_into().unwrap();
        let user_name = entity.user_name.clone().try_into().unwrap();
        let password  = entity.password.clone().try_into().unwrap();
        let mail      = entity.mail.clone().try_into().unwrap();
        let model = Model{id , user_id , user_name , password , mail};
        Ok(model)
    }
    // 他のモデルからEntityへ変換する
    fn from_model(model: &Model) -> Result<User>{
        let user = User::build(model.user_id.to_owned(), 
                                    model.user_name.to_owned(), 
                                    model.password.to_owned(), 
                                    model.mail.to_owned());
        Ok(user)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn from_entity() {
        let user =  User::build(
            String::from("5772a800-fef1-40bf-888b-68fddd29d881"),
            String::from("user001"),
            String::from("a034408b78dfee92cdbfc6e5247cf0ece119f30e6ba7653f4b7a6f2f384f92a3c7cd4a0ec914ae3fb1ea93684b46f8ff2644ec0198d67be2fd2cbf68587f07b8"),
        String::from("yamada@sample.com"));
        let result = UserConverterSeaOrm::from_entity(&user).unwrap();
        println!("{:?}" , result);
        assert!(true);
    }
    #[test]
    fn from_model() {
        let model =  Model{
            id:0,
            user_id:String::from("5772a800-fef1-40bf-888b-68fddd29d881"),
            user_name:String::from("user0001"),
            password:String::from("a034408b78dfee92cdbfc6e5247cf0ece119f30e6ba7653f4b7a6f2f384f92a3c7cd4a0ec914ae3fb1ea93684b46f8ff2644ec0198d67be2fd2cbf68587f07b8"),
            mail:String::from("yamada@sample.com")};
        let result = UserConverterSeaOrm::from_model(&model).unwrap();
        println!("{:?}" , result);
        assert!(true);
    }
}