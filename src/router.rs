use crate::{
    configuration::ApplicationSettings,
    cors::get_cors_settings,
    handler::{login_handler, me_handler, register_handler, status_handler},
};
use axum::{
    extract::FromRef,
    routing::{get, post},
    Router,
};
use sea_orm::DatabaseConnection;
use tower_http::trace::TraceLayer;

#[derive(Clone)]
pub struct Secrets {
    pub hash_secret: String,
    pub jwt_secret: String,
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub db_connection: DatabaseConnection,
    pub secrets: Secrets,
}

pub fn make_router(
    db_connection: DatabaseConnection,
    app_settings: &ApplicationSettings,
) -> Router {
    // Innit shared state
    let state = AppState {
        db_connection,
        secrets: Secrets {
            hash_secret: app_settings.hash_secret.clone(),
            jwt_secret: app_settings.jwt_secret.clone(),
        },
    };
    // Create axum router
    let user_routes = Router::new()
        .route("/register", post(register_handler))
        .route("/login", get(login_handler))
        .route("/me", get(me_handler));

    let api_routes = Router::new().nest("/user", user_routes).with_state(state);

    let cors_layer = get_cors_settings(app_settings);

    Router::new()
        .route("/health_check", get(status_handler))
        .nest("/api", api_routes)
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
}
