use actix_web::{App, HttpServer, middleware, web};
use actix_web::web::{resource, ServiceConfig};
use chapter17::handlers::handl_func;
use chapter17::handlers::tera_handler;
use tera::Tera;
    use openssl::ssl::{SslAcceptor, SslAcceptorBuilder, SslFiletype, SslMethod};
/// ## 17-3.リクエストハンドラ
/// ### リスト17-6 route()メソッド
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ロガーを初期化する
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    // Template Engine Teraを生成する
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/views/**/*")).unwrap();
     // Cookieセッションの準備 ランダムな署名/暗号化キーを生成
     let secret_key = actix_web::cookie::Key::generate();
     // RedisSessionStoreを生成する
     let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379").await.unwrap();
    // HttpServerの起動 
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // ロギングミドルウェアの登録
            .app_data(web::Data::new(tera.clone())) // Teraの登録
            .configure(set_configure) //  パスとハンドラのマッピング
    }).bind_openssl("127.0.0.1:8080", create_ssl_acceptor_builder())?.run().await
    //.bind("127.0.0.1:8080")?.run().await
}
// パスとハンドラのマッピング定義
fn set_configure(cfg: &mut ServiceConfig) -> () {
    cfg.service(
        web::scope("/sample") // スコープを設定する
        .service(
            resource("/calc")  // パスパターン設定する
                // パスパターンに対するハンドラを設定する
                .route(web::get().to(handl_func::calc_2))
                .route(web::post().to(handl_func::calc_2)))
        // Teraを利用するパス設定
        .service(resource("/calc_form")
                .route(web::get().to(tera_handler::calc_get))
                .route(web::get().to(tera_handler::calc_post)))
    );
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

