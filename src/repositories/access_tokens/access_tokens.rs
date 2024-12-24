use crate::models::access_token::AccessToken;
use chrono::{DateTime, Local};
use sqlx::{query, PgPool, Row};
use uuid::Uuid;

pub async fn create_access_token(
    token: &String,
    user_id: Uuid,
    expires_at: DateTime<Local>,
    pool: &PgPool,
) -> anyhow::Result<AccessToken, anyhow::Error> {
    let result =
        query("INSERT INTO access_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING id, user_id, token, created_at, updated_at, expires_at")
            .bind(user_id)
            .bind(token)
            .bind(expires_at)
            .fetch_one(pool)
            .await?;

    let access_token = AccessToken {
        id: result.get("id"),
        user_id: result.get("user_id"),
        token: result.get("token"),
        created_at: result.get("created_at"),
        updated_at: result.get("updated_at"),
        expires_at: result.get("expires_at"),
    };

    Ok(access_token)
}
