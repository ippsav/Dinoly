use axum::http::{header, HeaderValue, Method};
use tower_http::cors::{Any, CorsLayer};

use crate::configuration::ApplicationSettings;

pub fn get_cors_settings(settings: &ApplicationSettings) -> CorsLayer {
    let mut cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE]);
    cors = match settings.cors_origin.as_str() {
        "any" => cors.allow_origin(Any),
        _ => cors.allow_origin(settings.cors_origin.parse::<HeaderValue>().unwrap()),
    };
    cors
}
