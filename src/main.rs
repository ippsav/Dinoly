use std::net::TcpListener;

use lib::{configuration::GlobalConfig, server::run, telemetry::init_telemetry};
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let port = std::env::var("PORT").unwrap();
    dbg!(port);
    let exposed_port = std::env::var("APP_APPLICATION__PORT").unwrap();
    dbg!(exposed_port);
    // ini trace layer
    init_telemetry();
    // build config
    let env = std::env::var("ENVIRONMENT").ok();
    let config_path = std::env::current_dir()?.join("config");
    let config = GlobalConfig::build(env, config_path)?;
    // make listener
    let listener = TcpListener::bind(config.application.address())?;

    // run server
    tracing::debug!("listening on {}", config.application.address());
    Ok(run(listener).await?)
}

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("hyper server error")]
    Hyper(#[from] hyper::Error),
    #[error("parsing config error")]
    Config(#[from] config::ConfigError),
}
