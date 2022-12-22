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

impl ApiResponseError {
    pub fn simple_error(msg: &'static str) -> Self {
        Self::Simple(msg.into())
    }
    pub fn complicated_error(msg: &'static str, error: impl Serialize + 'static) -> Self {
        Self::Complicated {
            message: msg.into(),
            error: Box::new(error),
        }
    }
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
