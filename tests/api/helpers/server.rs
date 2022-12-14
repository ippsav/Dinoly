use std::net::TcpListener;

use hyper::{client::HttpConnector, Body, Client, Method, Request};
use lib::{
    configuration::{DatabaseSettings, GlobalConfig},
    router,
};

use migration::{Migrator, MigratorTrait};

use sea_orm::{
    prelude::Uuid, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement,
};
use serde_json::{json, Value};

use super::ParseJson;

#[derive(Debug)]
pub struct TestApp {
    pub config: GlobalConfig,
    pub database: DatabaseConnection,
    pub client: Client<HttpConnector>,
}

impl TestApp {
    pub async fn new() -> Self {
        // Parsing config
        let path = std::env::current_dir()
            .expect("couldn't get current directory")
            .join("config");

        let mut config =
            GlobalConfig::build(Some("test".into()), path).expect("couldn't build config");

        // Setup database
        let db = Self::setup_db(&mut config.database).await;

        Self {
            config,
            database: db,
            client: Client::new(),
        }
    }

    async fn setup_db(db_config: &mut DatabaseSettings) -> DatabaseConnection {
        let mut opt = DatabaseSettings::get_connection_options(&db_config.get_connection_string());

        let mut db = Database::connect(opt)
            .await
            .expect("couldn't connect to database");

        db_config.db_name = Uuid::new_v4().to_string();

        // create database
        let stmt = Statement::from_string(
            DatabaseBackend::Postgres,
            format!(r#"CREATE DATABASE "{}""#, db_config.db_name),
        );
        db.execute(stmt).await.expect("couldn't create database");

        opt = DatabaseSettings::get_connection_options(&db_config.get_connection_string());

        db = Database::connect(opt)
            .await
            .expect("couldn't connect to database");

        Migrator::up(&db, None)
            .await
            .expect("couldn't run the migrations");

        db
    }

    pub async fn spawn_server(&mut self) {
        // Create tcp listener
        let listener = TcpListener::bind(format!("{}:0", &self.config.application.address))
            .expect("couldn't create tcp listener");
        let local_addr = listener
            .local_addr()
            .expect("couldn't get local address from listener");
        let router = router::make_router(self.database.clone(), &self.config.application);

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(router.into_make_service())
                .await
                .unwrap()
        });

        self.config.application.port = local_addr.port();
    }

    pub fn get_http_uri(&self, path: Option<&str>) -> String {
        let path = path.unwrap_or("");

        format!("http://{}{}", &self.config.application.address(), path)
    }

    pub async fn login_user(&self, username: &str, password: &str) -> String {
        let user_input = json!({
            "username": username,
            "password": password,
        });
        // Create request
        let req = Request::builder()
            .method(Method::POST)
            .uri(self.get_http_uri(Some("/api/user/login")))
            .header("Content-Type", "application/json")
            .body(Body::from(user_input.to_string()))
            .expect("couldn't create request");

        // Sending request

        let res = self
            .client
            .request(req)
            .await
            .expect("coudn't send request");

        let body: Value = res.json_from_body().await.expect("couldn't json from body");

        body["data"]["token"]
            .as_str()
            .expect("couldn't parse token")
            .to_owned()
    }
}
