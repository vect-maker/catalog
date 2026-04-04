use axum::http::{HeaderValue, Method, header};
use tower_http::cors::CorsLayer;

pub fn get_cors_layer(client_url: &str) -> CorsLayer {
    CorsLayer::new()
        .allow_origin(client_url.parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
}
