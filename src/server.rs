use std::net::TcpListener;

use axum::{routing::IntoMakeService, Router, Server};
use hyper::{server::conn::AddrIncoming, Error};

use crate::router::make_router;

fn make_server(
    listener: TcpListener,
) -> Result<Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    // make router
    let router = make_router();

    // Start server
    Ok(Server::from_tcp(listener)?.serve(router.into_make_service()))
}

pub async fn run(listener: TcpListener) -> Result<(), Error> {
    let server = make_server(listener);

    server?.await
}
