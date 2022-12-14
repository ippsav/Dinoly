use std::fmt::Debug;

use axum::extract::State;
use axum::{extract::Json, http::StatusCode};
use sea_orm::prelude::Uuid;
use sea_orm::ActiveValue::Set;
use sea_orm::{query::Condition, ActiveModelTrait, EntityTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::entity::sea_orm_active_enums::Provider;
use crate::entity::user;
use crate::handler::helpers::{ApiResponse, ApiResponseError, ErrorToResponse, ResponseError};
use crate::handler::utils::{encode_jwt, hash_password};
use crate::router::Secrets;

// Client data to create a User
#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUserInput {
    #[validate(length(min = 6, max = 20))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 5, max = 25))]
    pub password: String,
}

// Response Object
#[derive(Serialize, Debug)]
pub struct RegisterResponseObject {
    token: String,
}

// Errors
#[derive(Debug)]
pub enum ApiError {
    BadClientData(ValidationErrors),
    UserAlreadyRegistered,
    DbInternalError,
    HashingError,
    JWTEncodingError,
}

impl ErrorToResponse for ApiError {
    fn into_api_response<T: Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::BadClientData(err) => ApiResponse::Error {
                error: ApiResponseError::complicated_error(
                    "invalid data from client",
                    ResponseError::from(err),
                ),
                status: StatusCode::BAD_REQUEST,
            },
            ApiError::UserAlreadyRegistered => ApiResponse::Error {
                error: ApiResponseError::simple_error("user already registered"),
                status: StatusCode::FORBIDDEN,
            },
            ApiError::DbInternalError | ApiError::HashingError | ApiError::JWTEncodingError => {
                ApiResponse::StatusCode(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

#[tracing::instrument(skip(secrets))]
pub async fn register_handler(
    State(db_connection): State<DatabaseConnection>,
    State(secrets): State<Secrets>,
    Json(created_user): Json<RegisterUserInput>,
) -> ApiResponse<RegisterResponseObject> {
    // Validating user input
    if let Err(error) = created_user.validate().map_err(ApiError::BadClientData) {
        return error.into_api_response();
    };

    // Check if user is already registered
    match user::Entity::find()
        .filter(
            Condition::any()
                .add(user::Column::Username.eq(created_user.username.clone()))
                .add(user::Column::Email.eq(created_user.email.clone())),
        )
        .one(&db_connection)
        .await
    {
        Ok(user) => {
            if user.is_some() {
                return ApiError::UserAlreadyRegistered.into_api_response();
            }
        }
        Err(_) => {
            return ApiError::DbInternalError.into_api_response();
        }
    };

    // Hash password

    let hashed_password = match hash_password(
        secrets.hash_secret.as_bytes(),
        created_user.password.as_bytes(),
    )
    .map_err(|_| ApiError::HashingError)
    {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    // Creating User model and inserting it
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set(created_user.username),
        email: Set(created_user.email),
        password_hash: Set(Some(hashed_password)),
        provider: Set(Provider::Local),
        ..Default::default()
    };

    let user: user::Model = match user
        .insert(&db_connection)
        .await
        .map_err(|_| ApiError::DbInternalError)
    {
        Ok(value) => value,
        Err(err) => {
            return err.into_api_response();
        }
    };

    // Creating the jwt token
    let token = match encode_jwt(secrets.jwt_secret.as_bytes(), &user.id)
        .map_err(|_| ApiError::JWTEncodingError)
    {
        Ok(value) => value,
        Err(err) => return err.into_api_response(),
    };

    ApiResponse::Data {
        data: RegisterResponseObject { token },
        status: StatusCode::OK,
    }
}
