use assert_json_diff::{assert_json_eq, assert_json_include};
use hyper::{Body, Method, Request};
use serde_json::{json, Value};

use crate::{
    helpers::{server::TestApp, ParseJson, TestCase},
    seeds::users::seed_one_local_user,
};

#[tokio::test]
async fn create_link_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, password) =
        seed_one_local_user(&app.database, &app.config.application.hash_secret).await;
    // Get token by logging in
    let token = app.login_user(&user.username, &password).await;

    // Link input to register a user
    let name = "link_name";
    let slug = "link_slug";
    let redirect_to = "http://google.com";

    let create_link_input = json!({
        "name": name,
        "slug":slug,
        "redirect_to": redirect_to,
    });

    // Create request
    let req = Request::builder()
        .uri(app.get_http_uri(Some("/api/links")))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::from(create_link_input.to_string()))
        .expect("couldn't create request");

    // Send request
    let res = app
        .client
        .request(req)
        .await
        .expect("coudln't send request");
    // Checking server response
    assert!(res.status().is_success());

    let body: Value = res
        .json_from_body()
        .await
        .expect("couldn't get json from body");

    assert!(body["error"].is_null());

    let data: Value = body["data"].to_owned();
    let expected_data = json!({
        "link": {
            "name": name,
            "slug": slug,
            "redirect_to": redirect_to,
            "owner_id": user.id,
        }
    });
    assert_json_include!(actual: data, expected: expected_data);
}

#[tokio::test]
async fn create_link_handler_with_bad_client_data() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, password) =
        seed_one_local_user(&app.database, &app.config.application.hash_secret).await;
    // Get token by logging in
    let token = app.login_user(&user.username, &password).await;

    let inputs_with_errors = vec![
        (
            json!({
                    "name": "link_name",
                    "slug": "link_slug",
                    "redirect_to": "bad url"
            }),
            json!({
                "message": "invalid data from client",
                "error": {
                    "fields": {
                        "redirect_to": "invalid url"
                    }
                }
            }),
        ),
        (
            json!({
                    "name": "wd",
                    "slug": "link_slug",
                    "redirect_to": "bad url"
            }),
            json!({
                "message": "invalid data from client",
                "error": {
                    "fields": {
                        "redirect_to": "invalid url",
                        "name": "invalid length"
                    }
                }
            }),
        ),
        (
            json!({
                    "name": "wd",
                    "slug": "wd",
                    "redirect_to": "bad url"
            }),
            json!({
                "message": "invalid data from client",
                "error": {
                    "fields": {
                        "redirect_to": "invalid url",
                        "name": "invalid length",
                        "slug": "invalid length"
                    }
                }
            }),
        ),
    ];

    let test_cases = TestCase::gen_test_cases(&inputs_with_errors);

    for test_case in test_cases.into_iter() {
        let req = Request::builder()
            .uri(app.get_http_uri(Some("/api/links")))
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {token}"))
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
async fn create_link_handler_with_bad_jwt_token() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, password) =
        seed_one_local_user(&app.database, &app.config.application.hash_secret).await;
    // Get token by logging in
    let token = app.login_user(&user.username, &password).await;

    // Link input to register a user
    let name = "link_name";
    let slug = "link_slug";
    let redirect_to = "bad url";

    let create_link_input = json!({
        "name": name,
        "slug":slug,
        "redirect_to": redirect_to,
    });

    // Create request
    let req = Request::builder()
        .uri(app.get_http_uri(Some("/api/links")))
        .method(Method::POST)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer e{token}"))
        .body(Body::from(create_link_input.to_string()))
        .expect("couldn't create request");

    // Send request
    let res = app
        .client
        .request(req)
        .await
        .expect("coudln't send request");
    // Checking server response
    assert!(res.status().is_client_error());
}
