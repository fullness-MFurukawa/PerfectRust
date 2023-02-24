///
/// 18章 外部クレート活用
/// 


use async_trait::async_trait;
use crate::Result;
use crate::domain::user::{user_name::UserName,user::User};
/// ## 18-5 アプリケーションの構成
/// ### リスト18-24 ユーザーを扱うRepositoryトレイト
#[async_trait]
pub trait UserRepository : Send + Sync {
    type Transaction;
    /// 指定されたユーザー名で問合せする
    async fn select_by_name(&self , _: &Self::Transaction, user_name: &UserName) -> Result<Option<User>>;
}