use std::net::TcpListener;

use lib::server::run;
use thiserror::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // make listener
    let listener = TcpListener::bind("0.0.0.0:3000")?;

    // run server

    Ok(run(listener).await?)
}

#[derive(Error, Debug)]
enum Error {
    #[error("io error")]
    IO(#[from] std::io::Error),
    #[error("hyper server error")]
    Hyper(#[from] hyper::Error),
}
