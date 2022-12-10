use std::str::FromStr;

use crate::{
    dto::user::User,
    entity::user::Entity as Users,
    handler::{
        helpers::{ApiResponse, ErrorToResponse},
        utils::decode_jwt,
    },
    router::Secrets,
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use hyper::StatusCode;
use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait};
use serde::Serialize;

pub enum ApiError {
    InvalidJwtToken,
    InvalidIdFormat,
    DbInternalError,
    UserNotFound,
}

#[derive(Serialize)]
pub struct MeResponse {
    pub user: User,
}

impl ErrorToResponse for ApiError {
    fn into_api_response<T: serde::Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::InvalidJwtToken | ApiError::InvalidIdFormat => {
                ApiResponse::StatusCode(StatusCode::NOT_ACCEPTABLE)
            }
            ApiError::DbInternalError => ApiResponse::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::UserNotFound => ApiResponse::StatusCode(StatusCode::BAD_REQUEST),
        }
    }
}

#[tracing::instrument(skip(secrets))]
pub async fn me_handler(
    State(db_connection): State<DatabaseConnection>,
    State(secrets): State<Secrets>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> ApiResponse<MeResponse> {
    let token = token.token();

    let claims = match decode_jwt(secrets.jwt_secret.as_bytes(), token)
        .map_err(|_| ApiError::InvalidJwtToken)
    {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    let user_id = match Uuid::from_str(&claims.sub).map_err(|_| ApiError::InvalidIdFormat) {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    let res = match Users::find_by_id(user_id)
        .one(&db_connection)
        .await
        .map_err(|_| ApiError::DbInternalError)
    {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    let user = match res.ok_or(ApiError::UserNotFound) {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    ApiResponse::Data {
        data: MeResponse { user: user.into() },
        status: StatusCode::OK,
    }
}
