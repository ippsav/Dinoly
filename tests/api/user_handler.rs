use assert_json_diff::assert_json_eq;
use hyper::{Body, Method, Request};
use serde_json::{json, Value};

use sea_orm::{query::Condition, ColumnTrait, EntityTrait, QueryFilter};

use lib::entity::user;

use crate::helpers::testing::TestCase;
use crate::{
    helpers::{server::TestApp, ParseJson},
    seeds::users::seed_one_local_user,
};

#[tokio::test]
async fn register_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // User input to register a user
    let username = "test_user";
    let email = "test@email.com";

    let user_input = json!({
        "username": username,
        "email": email,
        "password": "test_password",
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
async fn register_handler_with_bad_client_data() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    let test_cases = TestCase::gen_test_cases_from_file("register_user_handler_inputs");

    for test_case in test_cases.into_iter() {
        let req = Request::builder()
            .method(Method::POST)
            .uri(app.get_http_uri(Some("/api/user/register")))
            .header("Content-Type", "application/json")
            .body(Body::from(test_case.input.to_string()))
            .expect("couldn't create request");
        // Send request
        let res = app
            .client
            .request(req)
            .await
            .expect("coudln't send request");
        // Checking server response
        assert!(res.status().is_client_error());

        let body: Value = res
            .json_from_body()
            .await
            .expect("couldn't get json from body");

        assert!(body["data"].is_null());

        let error: Value = body["error"].to_owned();
        assert_json_eq!(error, test_case.error);
    }
}

#[tokio::test]
async fn login_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, password) =
        seed_one_local_user(&app.database, &app.config.application.hash_secret).await;

    let user_input = json!({
        "username": &user.username,
        "password": &password,
    });
    // Create request
    let req = Request::builder()
        .method(Method::POST)
        .uri(app.get_http_uri(Some("/api/user/login")))
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
    assert!(!token.is_empty());

    assert!(body["error"].is_null());
}

#[tokio::test]
async fn login_handler_with_bad_credentials() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, _) = seed_one_local_user(&app.database, &app.config.application.hash_secret).await;

    let user_input = json!({
        "username": &user.username,
        "password": "wrong_password",
    });
    // Create request
    let req = Request::builder()
        .method(Method::POST)
        .uri(app.get_http_uri(Some("/api/user/login")))
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
}

#[tokio::test]
async fn me_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, password) =
        seed_one_local_user(&app.database, &app.config.application.hash_secret).await;

    // Get token by logging in
    let token = app.login_user(&user.username, &password).await;
    // Create request
    let req = Request::builder()
        .method(Method::GET)
        .uri(app.get_http_uri(Some("/api/user/me")))
        .header("Authorization", &format!("Bearer {}", token))
        .body(Body::empty())
        .expect("couldn't create request");

    // Send request
    let res = app
        .client
        .request(req)
        .await
        .expect("couldn't send request");

    assert!(res.status().is_success());

    let body: Value = res
        .json_from_body()
        .await
        .expect("couldn't get body from response");

    assert!(body["error"].is_null());

    let data: Value = body["data"].to_owned();
    let expected_data = json!({
        "user": {
            "id": user.id,
            "username": user.username,
            "email": user.email,
            "provider": user.provider,
            "created_at": user.created_at,
            "updated_at": null
        }
    });
    assert_json_eq!(data, expected_data);
}
