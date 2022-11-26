use sea_orm::Database;
use std::net::TcpListener;

use lib::{
    configuration::{DatabaseSettings, GlobalConfig},
    server::run,
    telemetry::init_telemetry,
};
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // ini trace layer
    init_telemetry();
    // build config
    let env = std::env::var("ENVIRONMENT").ok();
    let config_path = std::env::current_dir()?.join("config");
    let config = GlobalConfig::build(env, config_path)?;
    // connect to database
    let connection_options =
        DatabaseSettings::get_connection_options(&config.database.get_connection_string());
    let db = Database::connect(connection_options).await?;
    // make listener
    let listener = TcpListener::bind(config.application.address())?;

    // run server
    tracing::debug!("listening on {}", config.application.address());
    Ok(run(listener, db).await?)
}

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("hyper server error")]
    Hyper(#[from] hyper::Error),
    #[error("parsing config error")]
    Config(#[from] config::ConfigError),
    #[error("error connecting to the database")]
    DBConnection(#[from] sea_orm::DbErr),
}
