use axum::async_trait;
use hyper::{Body, Response};
use serde::Deserialize;
use thiserror::Error;

#[async_trait]
pub trait ParseJson<T>
where
    for<'de> T: Deserialize<'de>,
{
    type Error;

    async fn json_from_body(mut self) -> Result<T, Self::Error>;
}

#[async_trait]
impl<T> ParseJson<T> for Response<Body>
where
    for<'de> T: Deserialize<'de>,
{
    type Error = ParseError;

    async fn json_from_body(self) -> Result<T, Self::Error> {
        let body = hyper::body::to_bytes(self.into_body())
            .await
            .map_err(|_| Self::Error::ParseBody)?;
        Ok(serde_json::from_slice(&body)?)
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("couldn't parse body into json")]
    ParseBody,
    #[error("couldn't deserialize body bytes into type")]
    Deserialize(#[from] serde_json::Error),
}
