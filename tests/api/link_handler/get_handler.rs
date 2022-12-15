use assert_json_diff::assert_json_include;
use hyper::{Body, Method, Request};
use lib::dto::url::Url;
use serde_json::{json, Value};

use crate::{
    helpers::{server::TestApp, ParseJson},
    seeds::{links::seed_links_for_user, users::seed_one_local_user},
};

#[tokio::test]
async fn get_links_handler_with_success() {
    // Run server
    let mut app = TestApp::new().await;
    app.spawn_server().await;

    // Seed database with one user
    let (user, password) =
        seed_one_local_user(&app.database, &app.config.application.hash_secret).await;
    // Seed database with a link to update
    let number_of_links = 5;
    let links = seed_links_for_user(&app.database, &user.id, number_of_links).await;
    // Get token by logging in
    let token = app.login_user(&user.username, &password).await;

    // Create request
    let path = &format!("/api/links?offset=0&limit={}", number_of_links);
    let req = Request::builder()
        .uri(app.get_http_uri(Some(path)))
        .method(Method::GET)
        .header("Authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .expect("couldn't create request");

    // Send request
    let res = app
        .client
        .request(req)
        .await
        .expect("coudln't send request");
    // Checking server response
    assert!(res.status().is_success());
    let links: Vec<Url> = links.into_iter().map(Into::into).collect();
    let body: Value = res
        .json_from_body()
        .await
        .expect("couldn't get json from body");

    assert!(body["error"].is_null());

    let data: Value = body["data"].to_owned();
    let expected_data = json!({ "links": links });
    assert_json_include!(actual: data, expected: expected_data);
}
