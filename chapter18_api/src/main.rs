
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use actix_web::{HttpServer, App, middleware, web};
use actix_web::web::{ServiceConfig,resource};
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use chapter18_lib::infrastructure::{sea_orm::pool::PoolSeaOrm, pool::Pool};
use chapter18_lib::application::provider::ServiceProvider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロガーの初期化
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // SeaORMのDatabaseConnectionの取得
    let pool:Arc<DatabaseConnection> = PoolSeaOrm::get().await.unwrap();
    // ServiceProviderの取得
    let provider = ServiceProvider::new();
    /*  サーバーの実行 */
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())        // ロギングミドルウェアの登録
            .app_data(web::Data::new(pool.clone()))     // SeaORMのDatabaseConnectionの登録
            .app_data(web::Data::new(provider.clone())) // Application Service Providerの登録
            .configure(set_config)  // サービスの登録
    }).bind_openssl("127.0.0.1:8082", create_ssl_acceptor_builder())?.run().await
}
///
/// OpenSSL SslAcceptorBuilderの生成
///
fn create_ssl_acceptor_builder() -> SslAcceptorBuilder {
    // OpenSSL構造を管理し、暗号スイート、セッションオプションなどを構成する
    let mut builder: SslAcceptorBuilder = SslAcceptor::mozilla_intermediate_v5(SslMethod::tls_server()).unwrap();
    // 秘密鍵の設定
    builder.set_private_key_file("localhost-key.pem", SslFiletype::PEM).unwrap();
    // 証明書の設定
    builder.set_certificate_chain_file("localhost.pem").unwrap();
    builder
}
///
///  パスとハンドラのマッピング定義
/// 
fn set_config(cfg: &mut ServiceConfig) -> () {
    use chapter18_api::handler::search::ProductSearchHandler;
    use chapter18_api::handler::register::ProductRegisterHandler;
    use chapter18_api::handler::authenticate::AuthenticateHandler;
    cfg.service(
        web::scope("/api_sample") // スコープを設定する
        .service(resource("/")  // パスパターン設定する
            // パスパターンに対するハンドラを設定する
            .route(web::post().to(AuthenticateHandler::authenticate)))
        .service(resource("/search/product")  // パスパターン設定する
            // パスパターンに対するハンドラを設定する
            .route(web::get().to(ProductSearchHandler::search)))
        .service(resource("/register/product")  // パスパターン設定する
            // パスパターンに対するハンドラを設定する
            .route(web::post().to(ProductRegisterHandler::register)))
    );
}