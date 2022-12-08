use assert_json_diff::assert_json_eq;
use hyper::{Body, Method, Request};
use serde_json::{json, Value};

use sea_orm::{query::Condition, ColumnTrait, EntityTrait, QueryFilter};

use lib::entity::user;

use crate::helpers::{server::TestApp, ParseJson};

#[tokio::test]
async fn register_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

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

    let response = app
        .client
        .request(req)
        .await
        .expect("couldn't send request");

    // Checking server response
    assert!(response.status().is_success());

    let body: Value = response
        .json_from_body()
        .await
        .expect("couldn't get json from body");

    let token = body["data"]["token"].to_string();

    assert!(body["error"].is_null());

    assert!(!token.is_empty());

    // Checking user creation
    let conditions = Condition::all()
        .add(user::Column::Username.eq(username))
        .add(user::Column::Email.eq(email));
    let created_user = user::Entity::find()
        .filter(conditions)
        .one(&app.database)
        .await
        .expect("couldn't query created user from database");
    assert!(created_user.is_some());
}

#[tokio::test]
async fn register_handler_with_bad_email() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // User input to register a user
    let username = "test_user";
    let email = "bad_email";

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

    let response = app
        .client
        .request(req)
        .await
        .expect("couldn't send request");

    // Checking server response
    assert!(response.status().is_client_error());

    let body: Value = response
        .json_from_body()
        .await
        .expect("couldn't get json from body");

    assert!(body["data"].is_null());

    let error_object = body["error"].to_owned();

    let expected_error = json!({
        "error": {
            "fields": {
                "email": "invalid email"
            }
        },
        "message": "invalid data from client",
    });

    assert_json_eq!(error_object, expected_error);
}

#[tokio::test]
async fn register_handler_with_bad_username() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // User input to register a user
    let username = "bad";
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

    let response = app
        .client
        .request(req)
        .await
        .expect("couldn't send request");

    // Checking server response
    assert!(response.status().is_client_error());

    let body: Value = response
        .json_from_body()
        .await
        .expect("couldn't get json from body");

    assert!(body["data"].is_null());

    let error_object = body["error"].to_owned();

    let expected_error = json!({
        "error": {
            "fields": {
                "username": "invalid length"
            }
        },
        "message": "invalid data from client",
    });

    assert_json_eq!(error_object, expected_error);
}

#[tokio::test]
async fn register_handler_with_bad_data() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Create client
    let client = hyper::Client::new();

    // User input to register a user
    let username = "bad";
    let email = "bad email";

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

    // Checking server response
    assert!(response.status().is_client_error());

    let body: Value = response
        .json_from_body()
        .await
        .expect("couldn't get json from body");

    assert!(body["data"].is_null());

    let error_object = body["error"].to_owned();

    let expected_error = json!({
        "error": {
            "fields": {
                "username": "invalid length",
                "email": "invalid email"
            }
        },
        "message": "invalid data from client",
    });

    assert_json_eq!(error_object, expected_error);
}
