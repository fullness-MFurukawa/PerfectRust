use std::sync::Arc;
use actix_web::{HttpServer, App, middleware, web};
use actix_web::web::{ServiceConfig,resource};
use actix_session::storage::RedisSessionStore;
use actix_session::SessionMiddleware;
use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
use sea_orm::DatabaseConnection;
use tera::Tera;
use chapter18_lib::infrastructure::sea_orm::pool::PoolSeaOrm;
use chapter18_lib::infrastructure::pool::Pool;
use chapter18_lib::application::provider::ServiceProvider;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロガーを初期化する
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Template Engine Teraを生成する
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
    // SeaORM DatabaseConnectionの取得
    let pool:Arc<DatabaseConnection> = PoolSeaOrm::get().await.expect("コネクションプールが取得できません。");
    // Applucation Service Providerの取得
    let provider = ServiceProvider::new();
    // Cookieセッションの準備 ランダムな署名/暗号化キーを生成
    let secret_key = actix_web::cookie::Key::generate();
    // RedisSessionStoreを生成する
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379").await.unwrap();
    HttpServer::new(move || {
        App::new()
        // セッションミドルウェアの登録(Radis)
        .wrap(SessionMiddleware::builder(
            redis_store.clone() ,   // RadisSessionStoreを指定する
            secret_key.clone())     // キーを指定する
            .cookie_name("rsession_id".to_string()).build())
        .wrap(middleware::Logger::default())    // ロギングミドルウェアの登録
        .app_data(web::Data::new(tera.clone())) // Teraの登録
        .app_data(web::Data::new(pool.clone())) // SeaORMのDatabaseConnectionの登録
        .app_data(web::Data::new(provider.clone())) // Application Service Providerの登録
        .configure(set_config)   // サービスの登録
    }).bind_openssl("127.0.0.1:8081", create_ssl_acceptor_builder())?.run().await
}

// OpenSSL SslAcceptorBuilderの生成
fn create_ssl_acceptor_builder() -> SslAcceptorBuilder {
    // OpenSSL構造を管理し、暗号スイート、セッションオプションなどを構成する
    let mut builder: SslAcceptorBuilder = 
    SslAcceptor::mozilla_intermediate_v5(SslMethod::tls_server()).unwrap();
    // 秘密鍵の設定
    builder.set_private_key_file("localhost-key.pem", SslFiletype::PEM).unwrap();
    // 証明書の設定
    builder.set_certificate_chain_file("localhost.pem").unwrap();
    builder
}

// パスとハンドラのマッピング定義
fn set_config(cfg: &mut ServiceConfig) -> () {
    use chapter18_web::handler::{search::SearchHandler,register::RegisterHandler,
                                 authenticate::AuthenticateHandler,menu_error::{ErrorHandler,MenuHandler}};
    cfg.service(
        web::scope("/web_sample") // スコープを設定する
        .service(resource("/search/product")  // パスパターン設定する
                // パスパターンに対するハンドラを設定する
                .route(web::get().to(SearchHandler::enter))
                .route(web::post().to(SearchHandler::result)))
        .service(resource("/register/product")
                .route(web::get().to(RegisterHandler::enter))
                .route(web::post().to(RegisterHandler::confirm)))
        .service(resource("/register/product/register")
                .route(web::get().to(RegisterHandler::complete)))
        .service(resource("/register/product/finish")
                .route(web::get().to(RegisterHandler::finish)))
        .service(resource("/login")  
                .route(web::get().to(AuthenticateHandler::enter))
                .route(web::post().to(AuthenticateHandler::authenticate)))
        .service(resource("/menu")  
                .route(web::get().to(MenuHandler::menu)))
        .service(resource("/error")  
                .route(web::get().to(ErrorHandler::error)))
    );
}
