use serde::Serialize;

// Global api response error struct
#[derive(Debug, Serialize)]
pub struct ApiResponseErrorObject<T>
where
    T: Serialize,
{
    pub message: String,
    pub error: Option<T>,
}

pub enum ApiResponseError<T>
where
    T: Serialize,
{
    Simple(String),
    Complicated { message: String, error: T },
}

impl<T> From<ApiResponseError<T>> for ApiResponseErrorObject<T>
where
    T: Serialize,
{
    fn from(val: ApiResponseError<T>) -> Self {
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

impl From<&'static str> for ApiResponseError<()> {
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
