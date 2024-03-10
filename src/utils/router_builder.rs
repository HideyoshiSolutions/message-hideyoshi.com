use axum::Router;
use http::Method;
use tower_http::cors::{AllowHeaders, AllowOrigin, CorsLayer};

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

    pub fn add_cors(&mut self, origins: Option<Vec<String>>) -> &mut Self {
        let allowed_origins = origins.map_or(AllowOrigin::any(), |origins| {
            AllowOrigin::list(origins.iter().map(|s| s.parse().unwrap()))
        });
        let allowed_headers = [
            "content-type".parse().unwrap(),
            "authorization".parse().unwrap(),
        ];

        let cors = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
            .allow_headers(allowed_headers)
            .allow_origin(allowed_origins);

        self.router = self.router.clone().layer(cors);
        return self;
    }

    pub fn build(&self) -> Router {
        self.router.clone()
    }
}
