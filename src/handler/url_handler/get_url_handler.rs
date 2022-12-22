use crate::{entity::url::{self, Entity as Link}, handler::helpers::ApiResponseData};
use axum::{extract::{Path, State}, http::StatusCode};
use sea_orm::{prelude::Uuid, DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use serde::Serialize;

use crate::{
    dto::url::Url,
    handler::{
        helpers::ApiResponse,
        utils::UserId,
    },
};


pub enum ApiError {
    LinkNotFound,
    ForbiddenRequest,
    DBInternalError,
}

impl<E> From<ApiError> for ApiResponseData<E>
    where
        E: Serialize + 'static,
{
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::LinkNotFound => ApiResponseData::error(None, "link not found", StatusCode::NOT_FOUND),
            ApiError::ForbiddenRequest => ApiResponseData::status_code(StatusCode::FORBIDDEN),
            ApiError::DBInternalError => ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR),
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
) -> ApiResponse<GetLinkResponse,()> {

    let link = Link::find_by_id(link_id)
        .filter(url::Column::DeletedAt.is_null())
        .one(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)?;

    let link: url::Model = link.ok_or(ApiError::LinkNotFound)?; 

    if link.owner_id != user_id {
        return Err(ApiError::ForbiddenRequest.into());
    };

    let data = GetLinkResponse {
        link: link.into(),
    };

    Ok(ApiResponseData::success_with_data(data, StatusCode::OK))
}
