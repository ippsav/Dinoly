use axum::Extension;
use axum::{http::StatusCode, Json};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::entity::sea_orm_active_enums::Provider;
use crate::entity::user;
use crate::handler::helpers::{ApiResponse, ApiResponseError, ErrorToResponse};
use crate::handler::utils::{encode_jwt, verify_password};
use crate::router::State;

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

impl ErrorToResponse for ApiError {
    fn into_api_response<T: serde::Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::UserNotFound => ApiResponse::Error {
                error: ApiResponseError::simple_error("user not found"),
                status: StatusCode::NOT_ACCEPTABLE,
            },
            ApiError::BadCredentials => ApiResponse::StatusCode(StatusCode::FORBIDDEN),
            ApiError::InternalError | ApiError::JWTEncodingError => {
                ApiResponse::StatusCode(StatusCode::INTERNAL_SERVER_ERROR)
            }
            ApiError::UserProviderNotValid => ApiResponse::Error {
                error: ApiResponseError::simple_error("bad provider"),
                status: StatusCode::BAD_REQUEST,
            },
        }
    }
}

pub async fn login_handler(
    Json(user_input): Json<LoginUserInput>,
    Extension(state): Extension<Arc<State>>,
) -> ApiResponse<LoginResponseObject> {
    let state = state.clone();

    let res = match user::Entity::find()
        .filter(user::Column::Username.eq(user_input.username))
        .one(&state.db_connection)
        .await
        .map_err(|_| ApiError::UserNotFound)
    {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    let user = match res {
        Some(value) => value,
        None => return ApiError::UserNotFound.into_api_response(),
    };

    let password = match user.provider {
        Provider::Google => return ApiError::UserProviderNotValid.into_api_response(),
        Provider::Local => user.password_hash,
    };

    let hashed_password = match password.ok_or(ApiError::InternalError) {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    let is_match = match verify_password(
        state.hash_secret.as_bytes(),
        user_input.password.as_bytes(),
        &hashed_password,
    )
    .map_err(|_| ApiError::InternalError)
    {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    if !is_match {
        return ApiError::BadCredentials.into_api_response();
    };

    // Creating the jwt token
    let token = match encode_jwt(state.jwt_secret.as_bytes(), &user.id)
        .map_err(|_| ApiError::JWTEncodingError)
    {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    ApiResponse::Data {
        data: LoginResponseObject { token },
        status: StatusCode::OK,
    }
}
