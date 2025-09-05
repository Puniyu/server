use std::net::IpAddr;
use actix_web::HttpServer;
use crate::info;
use crate::logger::log_init;
pub async fn run_server(host: Option<IpAddr>, port: Option<u16>) {
    log_init();
    let host = host.unwrap_or(IpAddr::V4([127, 0, 0, 1].into()));
    let port = port.unwrap_or(33720);
    let addr = format!("{}:{}", host, port);
    info!("服务器在 {} 运行", addr);
    HttpServer::new(|| {
        actix_web::App::new().service(actix_web::web::resource("/").to(|| async { "Hello World!" }))
    })
        .bind(addr)
        .unwrap()
        .run()
        .await
        .unwrap();
}
