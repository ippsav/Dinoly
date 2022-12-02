use std::collections::HashMap;
use std::fmt::Debug;

use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::dto::user::User;
use crate::handler::helpers::{ApiResponse, ApiResponseError, ErrorToResponse};

// Client data to create a User
#[derive(Debug, Validate, Deserialize)]
pub struct CreateUser {
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
    user: User,
}

// Errors
#[derive(Debug)]
pub enum ApiError {
    BadClientData(ValidationErrors),
}

impl ErrorToResponse for ApiError {
    fn into_api_response<T: Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::BadClientData(err) => ApiResponse::Error {
                error: ApiResponseError::Complicated {
                    message: "invalid data from client".into(),
                    error: Box::new(ResponseError::from(err)),
                },
                status: StatusCode::BAD_REQUEST,
            },
        }
    }
}

// Response error object
#[derive(Debug, Serialize)]
pub struct ResponseError {
    pub fields: Option<HashMap<String, String>>,
}

impl From<ValidationErrors> for ResponseError {
    fn from(v: ValidationErrors) -> Self {
        let mut hash_map: HashMap<String, String> = HashMap::new();
        v.field_errors().into_iter().for_each(|(k, v)| {
            let msg = format!("invalid {}", v[0].code);

            hash_map.insert(k.into(), msg);
        });
        Self {
            fields: Some(hash_map),
        }
    }
}

pub async fn register_handler(Json(created_user): Json<CreateUser>) -> ApiResponse<String> {
    if let Err(error) = created_user
        .validate()
        .map_err(|err| ApiError::BadClientData(err))
    {
        return error.into_api_response();
    };

    ApiResponse::Data {
        data: "user created".into(),
        status: StatusCode::OK,
    }
}
