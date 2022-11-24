use hyper::{Body, Method, Request};
use serde_json::{json, Value};

use crate::helpers::{server::TestApp, ParseJson};

#[tokio::test]
async fn health_route_status() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Create client
    let client = hyper::Client::new();

    // Create request
    let request = Request::builder()
        .method(Method::GET)
        .uri(app.get_http_uri(Some("/health_check")))
        .body(Body::empty())
        .expect("could not make request");
    // Send request
    let response = client
        .request(request)
        .await
        .expect("couldn't send request");

    let status = response.status();
    assert!(status.is_success());

    let body: Value = response
        .json_from_body()
        .await
        .expect("couldn't get json from body");
    let expected_body = json!({
        "status":"ok"
    });

    assert_eq!(expected_body, body);
}
