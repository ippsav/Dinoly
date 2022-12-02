use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use super::{ApiResponseError, ApiResponseErrorObject};

pub enum ApiResponse<T: Serialize> {
    Data {
        data: T,
        status: StatusCode,
    },
    Error {
        error: ApiResponseError,
        status: StatusCode,
    },
    StatusCode(StatusCode),
}

pub trait ErrorToResponse {
    fn into_api_response<T: Serialize>(self) -> ApiResponse<T>;
}

// Global api response struct
#[derive(Serialize)]
pub struct ApiResponseObject<T>
where
    T: Serialize,
{
    data: Option<T>,
    error: Option<ApiResponseErrorObject>,
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Data { data, status } => (
                status,
                Json(ApiResponseObject::<T> {
                    data: Some(data),
                    error: None,
                }),
            )
                .into_response(),
            ApiResponse::Error { error, status } => (
                status,
                Json(ApiResponseObject::<T> {
                    data: None,
                    error: Some(error.into()),
                }),
            )
                .into_response(),
            ApiResponse::StatusCode(status) => status.into_response(),
        }
    }
}
