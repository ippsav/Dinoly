use crate::{entity::url::{self, Entity as Link}, handler::helpers::ApiResponseData};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, EntityTrait, Set, QueryFilter, ColumnTrait};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{
    dto::url::Url,
    handler::{
        helpers::{ApiResponse, ResponseError},
        utils::UserId,
    },
};

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateLinkInput {
    #[validate(length(min = 4, max = 20))]
    pub name: Option<String>,
    #[validate(length(min = 4, max = 20))]
    pub slug: Option<String>,
    #[validate(url)]
    pub redirect_to: Option<String>,
}

pub enum ApiError {
    BadClientData(ValidationErrors),
    LinkNotFound,
    ForbiddenUpdate,
    DBInternalError,
}


impl From<ApiError> for ApiResponseData<ResponseError> {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::BadClientData(err) => ApiResponseData::error(Some(ResponseError::from(err)), "invalid data from client", StatusCode::BAD_REQUEST),
            ApiError::LinkNotFound => ApiResponseData::error(None, "link not found", StatusCode::NOT_FOUND),
            ApiError::ForbiddenUpdate => ApiResponseData::status_code(StatusCode::FORBIDDEN),
            ApiError::DBInternalError => ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UpdateLinkResponse {
    pub link: Url,
}

#[tracing::instrument]
pub async fn update_url_handler(
    UserId(user_id): UserId,
    Path(link_id): Path<Uuid>,
    State(db): State<DatabaseConnection>,
    Json(update_link): Json<UpdateLinkInput>,
) -> ApiResponse<UpdateLinkResponse, ResponseError> {
    update_link
        .validate()
        .map_err(ApiError::BadClientData)?;

    let link = Link::find_by_id(link_id)
        .filter(url::Column::DeletedAt.is_null())
        .one(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)?;

    let link: url::Model = link.ok_or(ApiError::LinkNotFound)?;

    if link.owner_id != user_id {
        return Err(ApiError::ForbiddenUpdate.into());
    };

    let mut link: url::ActiveModel = link.into();

    if let Some(name) = update_link.name {
        link.name = Set(name);
    }

    if let Some(slug) = update_link.slug {
        link.slug = Set(slug);
    }

    if let Some(redirect_to) = update_link.redirect_to {
        link.redirect_to = Set(redirect_to);
    }

    link.updated_at = Set(Some(Utc::now().naive_utc()));

    let updated_link = link
        .update(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)?;

    let data = UpdateLinkResponse {
        link: updated_link.into(),
    };

    Ok(ApiResponseData::success_with_data(data, StatusCode::OK))
}
