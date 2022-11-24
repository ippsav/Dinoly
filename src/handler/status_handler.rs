use axum::Json;
use serde_json::{json, Value};

pub async fn status_handler() -> Json<Value> {
    Json(json!({
        "status": "ok"
    }))
}
