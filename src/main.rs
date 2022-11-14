use axum::{routing::get, Json, Router};
use serde_json::{json, Value};

async fn health_check_handler() -> Json<Value> {
    Json(json!({
        "status": "ok"
    }))
}

#[tokio::main]
async fn main() {
    // Create axum router
    let app = Router::new().route("/health_check", get(health_check_handler));

    // Start server
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
