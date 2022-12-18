use axum::http::StatusCode;
use hyper::{Body, Method, Request};

use crate::{
    helpers::server::TestApp,
    seeds::{links::seed_one_link_for_user, users::seed_one_local_user},
};

#[tokio::test]
async fn delete_link_handler_with_success() {
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

    // Create request
    let path = &format!("/api/links/{}", &link.id);
    let mut req = Request::builder()
        .uri(app.get_http_uri(Some(path)))
        .method(Method::DELETE)
        .header("Authorization", format!("Bearer {}",&token))
        .body(Body::empty())
        .expect("couldn't create request");

    // Send request
    let mut res = app
        .client
        .request(req)
        .await
        .expect("coudln't send request");

    // Checking server response
    assert!(res.status().is_success());

    // Creating request to check if the link is deleted
    req = Request::builder()
        .uri(app.get_http_uri(Some(path)))
        .method(Method::GET)
        .header("Authorization", format!("Bearer {}",&token))
        .body(Body::empty())
        .expect("couldn't create request");

    // Send request
    res = app
        .client
        .request(req)
        .await
        .expect("coudln't send request");

    // Checking the status code is 404 (not found)
    assert_eq!(res.status(), StatusCode::NOT_FOUND);
}
