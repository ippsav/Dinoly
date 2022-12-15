use crate::{
    configuration::ApplicationSettings,
    cors::get_cors_settings,
    handler::{
        create_url_handler, get_url_list_handler, login_handler, me_handler, register_handler,
        status_handler, update_url_handler,
    },
};
use axum::{
    extract::FromRef,
    routing::{get, post, put},
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
        .route("/login", post(login_handler))
        .route("/me", get(me_handler));

    let links_route = Router::new()
        .route("/", post(create_url_handler))
        .route("/:link_id", put(update_url_handler))
        .route("/", get(get_url_list_handler));

    let api_routes = Router::new()
        .nest("/user", user_routes)
        .nest("/links", links_route)
        .with_state(state);

    let cors_layer = get_cors_settings(app_settings);

    Router::new()
        .route("/health_check", get(status_handler))
        .nest("/api", api_routes)
        .layer(cors_layer)
        .layer(TraceLayer::new_for_http())
}
