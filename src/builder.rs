use actix_web::dev::HttpServiceFactory;
use actix_web::{dev, http::Method, web, HttpResponse};
use std::fmt;
use std::sync::Arc;


#[derive(Clone)]
pub struct RouterBuilder {
    method: Method,
    path: String,
    prefix: Option<String>,
    handler: Arc<dyn Fn() -> HttpResponse + Send + Sync>,
}

impl fmt::Debug for RouterBuilder {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RouterBuilder")
            .field("method", &self.method)
            .field("path", &self.path)
            .field("prefix", &self.prefix)
            .finish()
    }
}

impl RouterBuilder {
    pub fn new(path: String, handler: Box<dyn Fn() -> HttpResponse + Send + Sync>) -> Self {
        Self {
            method: Method::GET,
            path,
            prefix: None,
            handler: Arc::new(handler),
        }
    }

    pub fn with_method(mut self, method: Method) -> Self {
        self.method = method;
        self
    }

    pub fn with_prefix(mut self, prefix: String) -> Self {
        self.prefix = Some(prefix);
        self
    }
}

impl HttpServiceFactory for RouterBuilder {
    fn register(self, config: &mut dev::AppService) {
        let path = self.path.clone();
        let method = self.method.clone();
        let handler = self.handler.clone();

        let mut resource = actix_web::Resource::new(path);

        match method {
            Method::GET => {
                resource = resource.route(web::get().to(move || {
                    let handler = handler.clone();
                    async move {
                        handler()
                    }
                }));
            }
            Method::POST => {
                resource = resource.route(web::post().to(move || {
                    let handler = handler.clone();
                    async move {
                        handler()
                    }
                }));
            }
            Method::PUT => {
                resource = resource.route(web::put().to(move || {
                    let handler = handler.clone();
                    async move {
                        handler()
                    }
                }));
            }
            Method::DELETE => {
                resource = resource.route(web::delete().to(move || {
                    let handler = handler.clone();
                    async move {
                        handler()
                    }
                }));
            }
            _ => {
                resource = resource.route(web::get().to(move || {
                    let handler = handler.clone();
                    async move {
                        handler()
                    }
                }));
            }
        }

        resource.register(config);
    }
}