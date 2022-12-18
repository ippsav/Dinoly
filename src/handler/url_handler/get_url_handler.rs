use crate::entity::url::{self, Entity as Link};
use axum::extract::{Path, State};
use hyper::StatusCode;
use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::Serialize;

use crate::{
    dto::url::Url,
    handler::{
        helpers::{ApiResponse, ApiResponseError, ErrorToResponse},
        utils::UserId,
    },
};


pub enum ApiError {
    LinkNotFound,
    ForbiddenRequest,
    DBInternalError,
}

impl ErrorToResponse for ApiError {
    fn into_api_response<T: Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::LinkNotFound => ApiResponse::Error {
                error: ApiResponseError::simple_error("link not found"),
                status: StatusCode::NOT_FOUND,
            },
            ApiError::ForbiddenRequest => ApiResponse::StatusCode(StatusCode::FORBIDDEN),
            ApiError::DBInternalError => ApiResponse::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GetLinkResponse{
    pub link: Url,
}

#[tracing::instrument]
pub async fn get_url_handler(
    UserId(user_id): UserId,
    Path(link_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
) -> ApiResponse<GetLinkResponse> {

    let link = match Link::find_by_id(link_id)
        .filter(url::Column::DeletedAt.is_null())
        .one(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)
    {
        Ok(v) => v,
        Err(err) => return err.into_api_response(),
    };

    let link: url::Model = match link.ok_or(ApiError::LinkNotFound) {
        Ok(v) => v,
        Err(err) => return err.into_api_response(),
    };

    if link.owner_id != user_id {
        return ApiError::ForbiddenRequest.into_api_response();
    };

    ApiResponse::Data {
        data: GetLinkResponse {
            link: link.into(),
        },
        status: StatusCode::OK,
    }
}
