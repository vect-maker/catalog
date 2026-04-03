use axum::http::{HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

pub fn get_cors_layer(client_url: &str) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(client_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_headers(Any)
}
