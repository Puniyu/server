use crate::{info, middleware};
use crate::logger::log_init;
use actix_web::middleware::{NormalizePath, TrailingSlash};
use actix_web::{App, HttpServer};
use std::net::IpAddr;
use crate::registry::ROUTER_REGISTRY;

pub async fn run_server(host: Option<IpAddr>, port: Option<u16>) {
	log_init();
	let host = host.unwrap_or(IpAddr::V4([127, 0, 0, 1].into()));
	let port = port.unwrap_or(33720);
	let addr = format!("{}:{}", host, port);
	info!("服务器在 {} 运行", addr);
	HttpServer::new(|| {
		let routers = ROUTER_REGISTRY.get_all();
		let mut app = App::new()
			.wrap(middleware::AccessLog)
			.wrap(NormalizePath::new(TrailingSlash::Trim))
			.service(actix_web::web::resource("/").to(|| async { "Hello World!" }));
		for router in routers {
			app = app.service(router);
		}
		app
	})
	.bind(addr)
	.unwrap()
	.run()
	.await
	.unwrap();
}

pub  async fn run_server_spawn(host: Option<IpAddr>, port: Option<u16>) {
	tokio::spawn(run_server(host, port));
}
