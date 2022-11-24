use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

use super::{ApiResponseError, ApiResponseErrorObject};

// type ApiResponse<T: Serialize + IntoResponse, E: Serialize + IntoResponse> = Result<T, E>;

pub enum ApiResponse<T: Serialize, E: Serialize> {
    Data {
        data: T,
        status: StatusCode,
    },
    Error {
        error: ApiResponseError<E>,
        status: StatusCode,
    },
}

// Global api response struct
#[derive(Serialize, Debug)]
pub struct ApiResponseObject<T, E>
where
    T: Serialize,
    E: Serialize,
{
    data: Option<T>,
    error: Option<ApiResponseErrorObject<E>>,
}

// impl<T, E> From<ApiResponse<T, E>> for ApiResponseObject<T, E>
// where
//     T: Serialize,
//     E: Serialize,
// {
//     fn from(res: ApiResponse<T, E>) -> Self {
//         match res {
//             ApiResponse::Data { data, status } => todo!(),
//             ApiResponse::Error { error, status } => todo!(),
//         }
//     }
// }

impl<T, E> IntoResponse for ApiResponse<T, E>
where
    T: Serialize,
    E: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        match self {
            ApiResponse::Data { data, status } => (
                status,
                Json(ApiResponseObject::<T, E> {
                    data: Some(data),
                    error: None,
                }),
            )
                .into_response(),
            ApiResponse::Error { error, status } => (
                status,
                Json(ApiResponseObject::<T, E> {
                    data: None,
                    error: Some(error.into()),
                }),
            )
                .into_response(),
        }
    }
}
