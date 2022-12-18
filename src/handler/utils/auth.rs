use std::str::FromStr;

use crate::entity::user::Entity as User;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts, State},
    headers::{authorization::Bearer, Authorization},
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    RequestPartsExt, TypedHeader,
};
use sea_orm::{prelude::Uuid, EntityTrait};

use crate::router::AppState;

use super::decode_jwt;

pub struct UserId(pub Uuid);

#[async_trait]
impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AuthError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Extract state
        let State(state) = parts
            .extract_with_state::<State<AppState>, S>(state)
            .await
            .map_err(|_| AuthError::InternalError)?;

        // Decode the user data
        let token_data = decode_jwt(state.secrets.jwt_secret.as_bytes(), bearer.token())
            .map_err(|_| AuthError::InvalidToken)?;

        let user_id = Uuid::from_str(&token_data.sub).map_err(|_| AuthError::InvalidToken)?;

        let user = User::find_by_id(user_id)
            .one(&state.db_connection)
            .await
            .map_err(|_| AuthError::InternalError)?;
        match user {
            Some(value) => Ok(UserId(value.id)),
            None => Err(AuthError::WrongCredentials),
        }
    }
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
    InternalError,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let status = match self {
            AuthError::WrongCredentials => StatusCode::UNAUTHORIZED,
            AuthError::MissingCredentials => StatusCode::BAD_REQUEST,
            AuthError::TokenCreation | AuthError::InternalError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            AuthError::InvalidToken => StatusCode::FORBIDDEN,
        };
        status.into_response()
    }
}
