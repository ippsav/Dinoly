use crate::entity::url::{self, Entity as Link};
use axum::{
    extract::{Path, State},
    Json,
};
use chrono::Utc;
use hyper::StatusCode;
use sea_orm::{prelude::Uuid, ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::{
    dto::url::Url,
    handler::{
        helpers::{ApiResponse, ApiResponseError, ErrorToResponse, ResponseError},
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
    LinkNoFound,
    ForbiddenUpdate,
    DBInternalError,
}

impl ErrorToResponse for ApiError {
    fn into_api_response<T: Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::BadClientData(err) => ApiResponse::Error {
                error: ApiResponseError::complicated_error(
                    "invalid data from client",
                    ResponseError::from(err),
                ),
                status: StatusCode::BAD_REQUEST,
            },
            ApiError::LinkNoFound => ApiResponse::Error {
                error: ApiResponseError::simple_error("link not found"),
                status: StatusCode::NOT_FOUND,
            },
            ApiError::ForbiddenUpdate => ApiResponse::StatusCode(StatusCode::FORBIDDEN),
            ApiError::DBInternalError => ApiResponse::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
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
) -> ApiResponse<UpdateLinkResponse> {
    if let Err(err) = update_link
        .validate()
        .map_err(|err| ApiError::BadClientData(err))
    {
        return err.into_api_response();
    };

    let link = match Link::find_by_id(link_id)
        .one(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)
    {
        Ok(v) => v,
        Err(err) => return err.into_api_response(),
    };

    let link: url::Model = match link.ok_or(ApiError::LinkNoFound) {
        Ok(v) => v,
        Err(err) => return err.into_api_response(),
    };

    if link.owner_id != user_id {
        return ApiError::ForbiddenUpdate.into_api_response();
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

    let updated_link = match link
        .update(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)
    {
        Ok(v) => v,
        Err(err) => return err.into_api_response(),
    };

    ApiResponse::Data {
        data: UpdateLinkResponse {
            link: updated_link.into(),
        },
        status: StatusCode::OK,
    }
}
