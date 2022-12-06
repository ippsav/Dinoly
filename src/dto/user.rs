use sea_orm::prelude::*;
use serde::Serialize;

use crate::entity::{sea_orm_active_enums::Provider, user};

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

#[derive(Serialize)]
pub struct Claims {
    pub sub: String,
    pub iat: DateTimeUtc,
    pub exp: DateTimeUtc,
}

impl From<user::Model> for User {
    fn from(v: user::Model) -> Self {
        Self {
            id: v.id,
            username: v.username,
            email: v.email,
            password_hash: v.password_hash,
            provider: v.provider,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}
