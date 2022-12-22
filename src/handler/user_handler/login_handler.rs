use axum::extract::State;
use axum::{http::StatusCode, Json};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};

use crate::entity::sea_orm_active_enums::Provider;
use crate::entity::user;
use crate::handler::helpers::{ApiResponse, ApiResponseData};
use crate::handler::utils::{encode_jwt, verify_password};
use crate::router::Secrets;

// Client input
#[derive(Deserialize, Debug)]
pub struct LoginUserInput {
    pub username: String,
    pub password: String,
}

// Response Object
#[derive(Serialize, Debug)]
pub struct LoginResponseObject {
    token: String,
}

pub enum ApiError {
    UserNotFound,
    BadCredentials,
    UserProviderNotValid,
    InternalError,
    JWTEncodingError,
}

impl<E> From<ApiError> for ApiResponseData<E>
    where
        E: Serialize + 'static,
{
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::UserNotFound => ApiResponseData::error(None, "user not found", StatusCode::NOT_ACCEPTABLE),
            ApiError::BadCredentials => ApiResponseData::status_code(StatusCode::FORBIDDEN),
            ApiError::UserProviderNotValid => ApiResponseData::error(None, "bad provider", StatusCode::BAD_REQUEST),
            ApiError::InternalError | ApiError::JWTEncodingError => ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[tracing::instrument(skip(secrets))]
pub async fn login_handler(
    State(secrets): State<Secrets>,
    State(db_connection): State<DatabaseConnection>,
    Json(user_input): Json<LoginUserInput>,
) -> ApiResponse<LoginResponseObject, ()> {
    let user = user::Entity::find()
        .filter(user::Column::Username.eq(user_input.username))
        .one(&db_connection)
        .await
        .map_err(|_| ApiError::UserNotFound)?;

    let user = user.ok_or(ApiError::UserNotFound)?;

    let password = match user.provider {
        Provider::Google => return Err(ApiError::UserProviderNotValid.into()),
        Provider::Local => user.password_hash,
    };

    let hashed_password = password.ok_or(ApiError::InternalError)?;

    let is_match = verify_password(
        secrets.hash_secret.as_bytes(),
        user_input.password.as_bytes(),
        &hashed_password,
    )
    .map_err(|_| ApiError::InternalError)?;

    if !is_match {
        return Err(ApiError::BadCredentials.into());
    };

    // Creating the jwt token
    let token = encode_jwt(secrets.jwt_secret.as_bytes(), &user.id)
        .map_err(|_| ApiError::JWTEncodingError)?;

    let data = LoginResponseObject { token };

    Ok(ApiResponseData::success_with_data(data, StatusCode::OK))
}
