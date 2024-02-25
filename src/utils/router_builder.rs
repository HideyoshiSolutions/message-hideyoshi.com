use axum::Router;

pub struct RouterBuilder {
    router: Router,
}

impl RouterBuilder {
    pub fn new() -> Self {
        RouterBuilder {
            router: Router::new(),
        }
    }

    pub fn add_config(&mut self, config_fn: fn(Router) -> Router) -> &mut Self {
        self.router = config_fn(self.router.clone());
        return self;
    }

    pub fn build(&self) -> Router {
        self.router.clone()
    }
}
