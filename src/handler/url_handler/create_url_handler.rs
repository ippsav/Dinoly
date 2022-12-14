use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{prelude::Uuid, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::{Condition, QueryFilter};
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationErrors};

use crate::handler::helpers::{ResponseError, ApiResponseData};
use crate::{
    dto::url::Url,
    entity::url,
    handler::{
        helpers::ApiResponse,
        utils::UserId,
    },
};

#[derive(Debug, Validate, Deserialize)]
pub struct CreateLinkInput {
    #[validate(length(min = 4, max = 20))]
    pub name: String,
    #[validate(length(min = 5, max = 20))]
    pub slug: String,
    #[validate(url)]
    pub redirect_to: String,
}

#[derive(Debug, Serialize)]
pub struct CreateLinkResponse {
    pub link: Url,
}

pub enum ApiError {
    BadClientData(ValidationErrors),
    DBInternalError,
    LinkExist,
}

impl From<ApiError> for ApiResponseData<ResponseError> {
    fn from(value: ApiError) -> Self {
        match value {
            ApiError::BadClientData(err) => ApiResponseData::error(Some(ResponseError::from(err)), "invalid data from client", StatusCode::BAD_REQUEST),
            ApiError::DBInternalError => ApiResponseData::status_code(StatusCode::INTERNAL_SERVER_ERROR),
            ApiError::LinkExist => ApiResponseData::error(None, "link with the name or slug provided already exists", StatusCode::BAD_REQUEST),
        }
    }
}

#[tracing::instrument]
pub async fn create_url_handler(
    UserId(user_id): UserId,
    State(db): State<DatabaseConnection>,
    Json(create_link): Json<CreateLinkInput>,
) -> ApiResponse<CreateLinkResponse, impl Serialize> {
    create_link.validate().map_err(ApiError::BadClientData)?;

    // Check if the user has a link with the same name or slug
    let conditions = Condition::any()
        .add(url::Column::Name.eq(create_link.name.clone()))
        .add(url::Column::Slug.eq(create_link.slug.clone()));

    match url::Entity::find()
        .filter(conditions)
        .one(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)
    {
        Ok(link) => {
            if link.is_some() {
                return Err(ApiError::LinkExist.into());
            }
        }
        Err(err) => return Err(err.into()),
    };

    let now = chrono::Utc::now();
    let link = url::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(create_link.name),
        slug: Set(create_link.slug.replace(' ', "")),
        redirect_to: Set(create_link.redirect_to),
        owner_id: Set(user_id),
        created_at: Set(now.naive_utc()),
        ..Default::default()
    };
    let link: url::Model = link
        .insert(&db)
        .await
        .map_err(|_| ApiError::DBInternalError)?;

    let data = CreateLinkResponse {
        link: link.into(),
    };

    Ok(ApiResponseData::success_with_data(data, StatusCode::OK))
}
