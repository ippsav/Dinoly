use crate::handler::health_check_handler;
use axum::{routing::get, Router};

pub fn make_router() -> Router {
    // Create axum router
    let router = Router::new().route("/health_check", get(health_check_handler));

    router
}
