use chrono::Duration;
use jsonwebtoken::{errors::Result, EncodingKey, Header};

use crate::{dto::user::Claims, entity::user};

pub fn encode_jwt(secret: &[u8], user: &user::Model) -> Result<String> {
    let now = chrono::Utc::now();

    let claims = Claims {
        sub: user.id.to_string(),
        iat: now,
        exp: now + Duration::hours(4),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret),
    )
}
