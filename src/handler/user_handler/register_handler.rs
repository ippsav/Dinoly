use std::collections::HashMap;
use std::fmt::Debug;

use axum::{extract::Json, http::StatusCode};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::dto::user::User;
use crate::handler::helpers::{ApiResponse, ApiResponseError};

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
// #[derive(Error, Debug)]
// pub enum ApiError {
//     #[error("error validating client data")]
//     BadClientData(#[from] ValidationErrors),
// }

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

// impl From<ApiError> for ApiResponseError<ResponseError> {
//     fn from(error: ApiError) -> Self {
//         match error {
//             ApiError::BadClientData(error) => ApiResponseError::Complicated {
//                 message: "error validating client input".into(),
//                 error: error.into(),
//             },
//         }
//     }
// }

pub async fn register_handler(
    Json(created_user): Json<CreateUser>,
) -> ApiResponse<String, ResponseError> {
    if let Err(error) = created_user.validate() {
        return ApiResponse::Error {
            error: ApiResponseError::Complicated {
                message: "invalid data from client".into(),
                error: error.into(),
            },
            status: StatusCode::BAD_REQUEST,
        };
    };

    ApiResponse::Data {
        data: "user created".into(),
        status: StatusCode::OK,
    }
}
