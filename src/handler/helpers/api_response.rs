use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use super::{ApiResponseError, ApiResponseErrorObject};

// Response types
pub enum ApiResponseType {
    SuccessWithData,
    StatusCodeOnly,
    Error,
}

impl Default for ApiResponseType {
    fn default() -> Self {
        Self::SuccessWithData
    }
}

pub enum ApiResponseData<T: Serialize> {
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

impl<T> ApiResponseData<T>
where
    T: Serialize + 'static,
{
    pub fn success_with_data(data: T, status: StatusCode) -> Self {
        Self::Data { data, status }
    }

    pub fn status_code(status: StatusCode) -> Self {
        Self::StatusCode(status)
    }

    pub fn error(error: Option<T>, message: &'static str, status: StatusCode) -> Self {
        match error {
            Some(err) => Self::Error {
                error: ApiResponseError::complicated_error(message, err),
                status,
            },
            None => Self::Error {
                error: ApiResponseError::simple_error(message),
                status,
            },
        }
    }
}

impl<T> IntoResponse for ApiResponseData<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponseData::Data { data, status } => (
                status,
                Json(ApiResponseObject::<T> {
                    data: Some(data),
                    error: None,
                }),
            )
                .into_response(),
            ApiResponseData::Error { error, status } => (
                status,
                Json(ApiResponseObject::<T> {
                    data: None,
                    error: Some(error.into()),
                }),
            )
                .into_response(),
            ApiResponseData::StatusCode(status) => status.into_response(),
        }
    }
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

pub type ApiResponse<T, E> = Result<ApiResponseData<T>, ApiResponseData<E>>;
