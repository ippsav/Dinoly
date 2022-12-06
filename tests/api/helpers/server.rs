use std::net::TcpListener;

use lib::{
    configuration::{DatabaseSettings, GlobalConfig},
    router,
};

use migration::{Migrator, MigratorTrait};

use sea_orm::{
    prelude::Uuid, ConnectionTrait, Database, DatabaseBackend, DatabaseConnection, Statement,
};

#[derive(Debug)]
pub struct TestApp {
    pub config: GlobalConfig,
    pub database: DatabaseConnection,
}

impl TestApp {
    pub async fn new() -> Self {
        // Parsing config
        let path = std::env::current_dir()
            .expect("couldn't get current directory")
            .join("config");

        let mut config =
            GlobalConfig::build(Some("test".into()), path).expect("couldn't build config");

        let db = Self::setup_db(&mut config.database).await;

        Self {
            config,
            database: db,
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
        let db = Self::setup_db(&mut self.config.database).await;
        let router = router::make_router(db.clone(), &self.config.application);

        tokio::spawn(async move {
            axum::Server::from_tcp(listener)
                .unwrap()
                .serve(router.into_make_service())
                .await
                .unwrap()
        });

        self.config.application.port = local_addr.port();
    }

    pub fn get_http_uri(&self, path: Option<&'static str>) -> String {
        let path = path.unwrap_or("");

        format!("http://{}{}", &self.config.application.address(), path)
    }
}
