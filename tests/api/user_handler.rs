use hyper::{Body, Method, Request};
use serde_json::{json, Value};

use crate::helpers::{server::TestApp, ParseJson};

#[tokio::test]
async fn register_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Create client
    let client = hyper::Client::new();

    // User input to register a user
    let username = "test_user";
    let email = "test@email.com";

    let user_input = json!({
        "username": username.to_owned(),
        "email": email.to_owned(),
        "password": "test_password".to_owned(),
    });
    // Create request
    let req = Request::builder()
        .method(Method::POST)
        .uri(app.get_http_uri(Some("/api/user/register")))
        .header("Content-Type", "application/json")
        .body(Body::from(user_input.to_string()))
        .expect("couldn't create request");

    let response = client.request(req).await.expect("couldn't send request");

    assert!(response.status().is_success());

    let body: Value = response
        .json_from_body()
        .await
        .expect("couldn't get json from body");

    let token = body["data"]["token"].to_string();

    assert!(body["error"].is_null());

    assert!(!token.is_empty());
}
