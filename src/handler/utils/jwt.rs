use chrono::Duration;
use jsonwebtoken::{decode, errors, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::prelude::Uuid;

use crate::dto::user::Claims;

pub fn encode_jwt(secret: &[u8], user_id: &Uuid) -> errors::Result<String> {
    let now = chrono::Utc::now();

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: (now + Duration::hours(4)).timestamp(),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}

pub fn decode_jwt(secret: &[u8], token: &str) -> errors::Result<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}
