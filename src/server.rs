use axum::{routing::IntoMakeService, Router, Server};
use hyper::{server::conn::AddrIncoming, Error};
use sea_orm::DatabaseConnection;
use std::net::TcpListener;

use crate::{configuration::GlobalConfig, router::make_router};

fn make_server(
    listener: TcpListener,
    config: GlobalConfig,
    db_connection: DatabaseConnection,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    // make router
    let router = make_router(config, db_connection);

    // Start server
    Ok(Server::from_tcp(listener)?.serve(router.into_make_service()))
}

pub async fn run(
    listener: TcpListener,
    config: GlobalConfig,
    db_connection: DatabaseConnection,
) -> Result<(), Error> {
    let server = make_server(listener, config, db_connection);

    server?.await
}
