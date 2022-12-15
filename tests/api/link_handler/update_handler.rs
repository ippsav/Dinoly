use assert_json_diff::assert_json_include;
use hyper::{Body, Method, Request};
use serde_json::{json, Value};

use crate::{
    helpers::{server::TestApp, ParseJson},
    seeds::{links::seed_one_link_for_user, users::seed_one_local_user},
};

#[tokio::test]
async fn update_link_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, password) =
        seed_one_local_user(&app.database, &app.config.application.hash_secret).await;
    // Seed database with a link to update
    let link = seed_one_link_for_user(&app.database, &user.id).await;
    // Get token by logging in
    let token = app.login_user(&user.username, &password).await;

    // Link input to register a user
    let name = "updated_name";
    let slug = "updated_slug";
    let redirect_to = "https://google.com";

    let create_link_input = json!({
        "name": name,
        "slug":slug,
        "redirect_to": redirect_to,
    });

    // Create request
    let path = &format!("/api/links/{}", &link.id);
    let req = Request::builder()
        .uri(app.get_http_uri(Some(path)))
        .method(Method::PUT)
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
