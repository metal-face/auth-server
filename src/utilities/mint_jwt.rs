use crate::models::claims::JWTClaims;
use crate::models::user::User;
use crate::repositories::access_tokens::access_tokens::create_access_token;
use chrono::DateTime;
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

pub async fn mint_jwt(user: &User, pool: &PgPool) -> String {
    let in_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let date_time = DateTime::from_timestamp((in_at + 1_209_600) as i64, 0);
    let expires_at = DateTime::from(date_time.unwrap());

    let my_claims = JWTClaims {
        jti: Uuid::new_v4().to_string(),
        aud: "client".to_owned(),
        sub: user.email.to_owned(),
        iss: "auth_server".to_owned(),
        first_name: user.first_name.to_owned(),
        last_name: user.last_name.to_owned(),
        client_id: Uuid::new_v4().to_string(),
        iat: in_at,
        exp: in_at + 1_209_600,
    };

    let jwt = encode(
        &Header::default(),
        &my_claims,
        &EncodingKey::from_secret(env::var("PRIVATE_KEY").unwrap().as_ref()),
    )
    .unwrap();

    create_access_token(&jwt, user.id, expires_at, pool)
        .await
        .unwrap();

    jwt
}
