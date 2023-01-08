use std::str::FromStr;

use crate::{
    dto::user::User,
    entity::user::Entity as Users,
    handler::{
        helpers::{ApiResponse, ApiResponseData, ResponseError},
        utils::decode_jwt,
    },
    router::Secrets,
};
use axum::{
    extract::State,
    headers::{authorization::Bearer, Authorization},
    http::StatusCode,
    TypedHeader,
};
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

impl<E> From<ApiError> for ApiResponseData<E>
    where
        E: Serialize + 'static,
{
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::InvalidJwtToken | ApiError::InvalidIdFormat => ApiResponseData::status_code(StatusCode::NOT_ACCEPTABLE),
            ApiError::DbInternalError => ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::UserNotFound => ApiResponseData::status_code(StatusCode::BAD_REQUEST),
        }
    }
}

#[tracing::instrument(skip(secrets))]
pub async fn me_handler(
    State(db_connection): State<DatabaseConnection>,
    State(secrets): State<Secrets>,
    TypedHeader(token): TypedHeader<Authorization<Bearer>>,
) -> ApiResponse<MeResponse, ResponseError> {
    let token = token.token();

    let claims = decode_jwt(secrets.jwt_secret.as_bytes(), token)
        .map_err(|_| ApiError::InvalidJwtToken)?;

    let user_id = Uuid::from_str(&claims.sub).map_err(|_| ApiError::InvalidIdFormat)?;

    let res = Users::find_by_id(user_id)
        .one(&db_connection)
        .await
        .map_err(|_| ApiError::DbInternalError)?;

    let user = res.ok_or(ApiError::UserNotFound)?;

    let data = MeResponse {
        user: user.into(),
    };

    Ok(
        ApiResponseData::success_with_data(data, StatusCode::OK)
    )
}
