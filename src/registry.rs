use crate::builder::RouterBuilder;
use std::sync::{Arc, LazyLock, RwLock};

#[derive(Debug, Clone, Default)]
pub struct RouterRegistry {
	routes: Arc<RwLock<Vec<RouterBuilder>>>,
}

unsafe impl Send for RouterRegistry {}
unsafe impl Sync for RouterRegistry {}

impl RouterRegistry {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn register(&self, registry: RouterBuilder) {
		if let Ok(mut routes) = self.routes.write() {
			routes.push(registry);
		}
	}

    pub fn get_all(&self) -> Vec<RouterBuilder> {
        if let Ok(routes) = self.routes.read() {
            (*routes).clone()
        } else {
            Vec::new()
        }
    }
}

pub static ROUTER_REGISTRY: LazyLock<RouterRegistry> = LazyLock::new(|| RouterRegistry::new());
