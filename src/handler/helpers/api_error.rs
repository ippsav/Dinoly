use erased_serde::Serialize as ErasedSerialize;
use serde::Serialize;

// Global api response error struct
#[derive(Serialize)]
pub struct ApiResponseErrorObject {
    pub message: String,
    pub error: Option<Box<dyn ErasedSerialize>>,
}

pub enum ApiResponseError {
    Simple(String),
    Complicated {
        message: String,
        error: Box<dyn ErasedSerialize>,
    },
}

impl From<ApiResponseError> for ApiResponseErrorObject {
    fn from(val: ApiResponseError) -> Self {
        match val {
            ApiResponseError::Simple(message) => Self {
                message,
                error: None,
            },
            ApiResponseError::Complicated { message, error } => Self {
                message,
                error: Some(error),
            },
        }
    }
}

impl From<&'static str> for ApiResponseError {
    fn from(message: &'static str) -> Self {
        Self::Simple(message.into())
    }
}

/*
    example 1:
    {
        "message": "simple error",
        "error": null
    }
    example 2:
    {
        "message": "complicated error",
        "error": {
            "code": "1213213",
            "foo": "bar"
        }
    }
*/
