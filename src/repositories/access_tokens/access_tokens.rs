use chrono::{DateTime, Local};
use sqlx::{query, PgPool};

struct AccessToken {
    id: String,
    token: String,
    user_id: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    expires_at: DateTime<Local>,
}

pub async fn create_access_token(
    access_token: String,
    user_id: String,
    expires_at: u64,
    pool: &PgPool,
) -> anyhow::Result<bool> {
    query("INSERT INTO access_tokens (token, user_id, expires_at) VALUES ($1, $2, $3)")
        .bind(access_token)
        .bind(user_id)
        .bind(expires_at)
        .execute(pool)
        .await?;

    Ok(true)
}
