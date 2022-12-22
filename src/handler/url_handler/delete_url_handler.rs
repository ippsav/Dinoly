use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait, IntoActiveModel, Set, ActiveModelTrait};
use crate::{entity::url::{self, Entity as Link}, handler::helpers::ApiResponseData};
use serde::Serialize;
use axum::{http::StatusCode, extract::{Path, State}};

use crate::handler::{helpers::ApiResponse, utils::UserId};


pub enum ApiError {
    LinkNotFound,
    ForbiddenDelete,
    DBInternalError,
}


impl<E> From<ApiError> for ApiResponseData<E> 
    where
        E: Serialize + 'static,
{
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::LinkNotFound => ApiResponseData::error(None, "link not found", StatusCode::NOT_FOUND),
            ApiError::ForbiddenDelete => ApiResponseData::StatusCode(StatusCode::FORBIDDEN),
            ApiError::DBInternalError => ApiResponseData::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}


#[tracing::instrument]
pub async fn delete_url_handler(
    UserId(user_id): UserId,
    Path(link_id): Path<Uuid>,
    State(db): State<DatabaseConnection>
) -> ApiResponse<(),()> {
    let link = Link::find_by_id(link_id)
        .one(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)?;

    let link: url::Model = link.ok_or(ApiError::LinkNotFound)?;
        
    if link.owner_id != user_id {
        return Err(ApiError::ForbiddenDelete.into());
    };

    let mut link_model = link.into_active_model();
    link_model.deleted_at = Set(Some(chrono::Utc::now().naive_utc()));

    link_model.update(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)?;
    
    Ok(ApiResponseData::status_code(StatusCode::OK))
}
