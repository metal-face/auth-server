use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{query, PgPool, Row};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct AccessToken {
    pub id: String,
    pub token: String,
    pub user_id: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub expires_at: DateTime<Local>,
}

pub async fn create_access_token(
    token: &String,
    user_id: String,
    expires_at: DateTime<Local>,
    pool: &PgPool,
) -> anyhow::Result<AccessToken, anyhow::Error> {
    let result =
        query("INSERT INTO access_tokens (user_id, token, expires_at) VALUES ($1, $2, $3)")
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
