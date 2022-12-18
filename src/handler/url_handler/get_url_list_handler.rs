use axum::extract::Query;
use axum::extract::State;
use hyper::StatusCode;
use sea_orm::QueryOrder;
use sea_orm::{Condition, ColumnTrait, DatabaseConnection, EntityTrait};
use sea_orm::{QueryFilter, QuerySelect};
use serde::{Deserialize, Serialize};

use crate::entity::url;
use crate::{
    dto::url::Url,
    handler::{
        helpers::{ApiResponse, ErrorToResponse},
        utils::UserId,
    },
};

#[derive(Debug, Serialize)]
pub struct GetLinkListResponse {
    pub links: Vec<Url>,
}

pub enum ApiError {
    DBInternalError,
}

impl ErrorToResponse for ApiError {
    fn into_api_response<T: Serialize>(self) -> ApiResponse<T> {
        match self {
            ApiError::DBInternalError => ApiResponse::StatusCode(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            offset: Some(0),
            limit: Some(10),
        }
    }
}

#[tracing::instrument]
pub async fn get_url_list_handler(
    UserId(user_id): UserId,
    params: Option<Query<Pagination>>,
    State(db): State<DatabaseConnection>,
) -> ApiResponse<GetLinkListResponse> {
    let Query(params) = params.unwrap_or_default();
    let conditions = Condition::all()
        .add(url::Column::OwnerId.eq(user_id))
        .add(url::Column::DeletedAt.is_null());
    let mut query = url::Entity::find()
        .filter(conditions)
        .order_by_desc(url::Column::CreatedAt);

    if let Some(limit) = params.limit {
        query = query.limit(limit);
    }

    if let Some(offset) = params.offset {
        query = query.offset(offset);
    }

    let links = match query.all(&db).await.map_err(|_| ApiError::DBInternalError) {
        Ok(v) => v,
        Err(err) => return err.into_api_response(),
    };

    ApiResponse::Data {
        data: GetLinkListResponse {
            links: links.into_iter().map(Into::into).collect(),
        },
        status: StatusCode::OK,
    }
}
