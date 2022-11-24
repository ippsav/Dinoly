use sea_orm::prelude::*;
use serde::Serialize;

use crate::entity::sea_orm_active_enums::Provider;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: Option<String>,
    pub provider: Provider,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}
