use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait, IntoActiveModel, Set, ActiveModelTrait};
use crate::entity::url::{self, Entity as Link};
use serde::Serialize;
use axum::{http::StatusCode, extract::{Path, State}};

use crate::handler::{helpers::{ApiResponse, ApiResponseError, ErrorToResponse}, utils::UserId};





pub enum ApiError {
    LinkNotFound,
    ForbiddenDelete,
    DBInternalError,
}

impl ErrorToResponse for ApiError {
    fn into_api_response<T: Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::LinkNotFound => ApiResponse::Error {
                error: ApiResponseError::simple_error("link not found"),
                status: StatusCode::NOT_FOUND,
            },
            ApiError::ForbiddenDelete => ApiResponse::StatusCode(StatusCode::FORBIDDEN),
            ApiError::DBInternalError => ApiResponse::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}


#[tracing::instrument]
pub async fn delete_url_handler(
    UserId(user_id): UserId,
    Path(link_id): Path<Uuid>,
    State(db): State<DatabaseConnection>
) -> ApiResponse<()> {
    let link = match Link::find_by_id(link_id)
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
        return ApiError::ForbiddenDelete.into_api_response();
    };

    let mut link_model = link.into_active_model();
    link_model.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));
    if let Err(err) = link_model.update(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)
    {
        return err.into_api_response();
    };
    
    ApiResponse::StatusCode(StatusCode::OK)
}
