use axum::{routing::IntoMakeService, Router, Server};
use hyper::{server::conn::AddrIncoming, Error};
use sea_orm::DatabaseConnection;
use std::net::TcpListener;

use crate::{configuration::GlobalConfig, router::make_router};

fn make_server(
    listener: TcpListener,
    db_connection: DatabaseConnection,
    config: &GlobalConfig,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    // make router
    let router = make_router(db_connection, &config.application);

    // Start server
    Ok(Server::from_tcp(listener)?.serve(router.into_make_service()))
}

pub async fn run(
    listener: TcpListener,
    db_connection: DatabaseConnection,
    config: &GlobalConfig,
) -> Result<(), Error> {
    let server = make_server(listener, db_connection, config);

    server?.await
}
