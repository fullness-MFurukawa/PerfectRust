use sea_orm::DbErr;
use thiserror::Error;
/// ## 18-5 アプリケーションの構成
/// ### リスト18-32 ドメイン層、インフラストラクチャ層、アプリケーション層で利用するエラー型
#[derive(Debug , Error)]
pub enum AppError{
    #[error("検索エラー:{0}")]
    SearchError(String) ,       // 検索処理エラー
    #[error("登録エラー:{0}")]
    RegisterError(String) ,     // 登録処理エラー
    #[error("認証エラー:{0}")]
    AuthenticateError(String) , // 認証エラー
    #[error(transparent)]
    InternalError(anyhow::Error) // 永続化層のエラー , ドメインルールエラー
}
impl From<DbErr> for AppError{ // SeaOrmのエラーをラップした内部エラーを生成する
    fn from(err: DbErr) -> Self {
        AppError::InternalError(anyhow::Error::new(err))
    }
}
impl From<String> for AppError{ // メッセージをラップした内部エラーを生成する
    fn from(msg: String) -> Self {
        AppError::InternalError(anyhow::Error::msg(msg))
    }
}
