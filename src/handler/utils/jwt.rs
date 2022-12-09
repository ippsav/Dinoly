use chrono::Duration;
use jsonwebtoken::{errors::Result, EncodingKey, Header};
use sea_orm::prelude::Uuid;

use crate::dto::user::Claims;

pub fn encode_jwt(secret: &[u8], user_id: &Uuid) -> Result<String> {
    let now = chrono::Utc::now();

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now,
        exp: now + Duration::hours(4),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}
