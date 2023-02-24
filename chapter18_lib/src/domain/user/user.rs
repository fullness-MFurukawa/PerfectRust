///
/// 18章 外部クレート活用
/// 

use uuid::Uuid;
use easy_hasher::easy_hasher::sha3_512;
use crate::Result;
use crate::domain::entity::Entity;
use super::{user_id::UserId, password::Password, mail::Mail, user_name::UserName};

/// ## 18-5 アプリケーションの構成
/// ### リスト18-23 User構造体
#[derive(PartialEq , Eq , Clone , Debug)]
pub struct User {
    pub user_id:    UserId      ,   // ユーザーId
    pub user_name:  UserName    ,   // ユーザー名
    pub password:   Password    ,   // パスワード
    pub mail:       Mail            // メールアドレス
}
impl User {
    /// ## Userを生成する
    /// ### user_idの生成、パスワードのハッシュ変換をする
    pub fn new(user_name: String , password:String , mail:String) -> Result<Self> {
        // uuidでユーザーIdを生成する
        let _user_id = Uuid::new_v4().to_string();
        // 受け取ったパスワードをSHA3-512でハッシュ変換する
        let _password = sha3_512(&password).to_hex_string();
        println!("password = {:?}" , &_password);
        // 値を生成した結果を返す
        Ok(Self {user_id: UserId::try_from(_user_id)? , user_name: UserName::try_from(user_name)?,
            password: Password::try_from(_password)? , mail:Mail::try_from(mail)?})
    }
    /// ## Userを再構築する
    /// ### user_idの生成はしない
    pub fn build(user_id: String , user_name:String , password: String , mail: String) -> Self{
        Self { user_id: UserId::try_from(user_id).unwrap(), 
               user_name: UserName::try_from(user_name).unwrap(), 
               password: Password::try_from(password).unwrap(), 
               mail: Mail::try_from(mail).unwrap() }

    }
}
impl Entity for User {}


