use sea_orm::prelude::*;
use serde::Serialize;

use crate::entity::url;

#[derive(Debug, Serialize)]
pub struct Url {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub redirect_to: String,
    pub owner_id: Uuid,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
}

impl From<url::Model> for Url {
    fn from(v: url::Model) -> Self {
        Self {
            id: v.id,
            name: v.name,
            slug: v.slug,
            redirect_to: v.redirect_to,
            owner_id: v.owner_id,
            created_at: v.created_at,
            updated_at: v.updated_at,
        }
    }
}
