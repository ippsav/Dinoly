use std::sync::Arc;

use crate::{
    configuration::GlobalConfig,
    handler::{login_handler, register_handler, status_handler},
};
use axum::{
    routing::{get, post},
    Extension, Router,
};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

#[derive(Debug)]
pub struct State {
    pub config: GlobalConfig,
    pub db_connection: DatabaseConnection,
}

pub fn make_router(config: GlobalConfig, db_connection: DatabaseConnection) -> Router {
    // Innit shared state
    let state = Arc::new(State {
        config,
        db_connection,
    });
    // Create axum router
    let user_routes = Router::new()
        .route("/register", post(register_handler))
        .route("/login", get(login_handler));

    let api_routes = Router::new()
        .nest("/user", user_routes)
        .layer(Extension(state));

    let router = Router::new()
        .route("/health_check", get(status_handler))
        .nest("/api", api_routes)
        .layer(TraceLayer::new_for_http());

    router
}
