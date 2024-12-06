use chrono::{DateTime, Local};
use sqlx::postgres::PgPool;
use sqlx::types::Json;
use sqlx::{query, Error};

#[derive(Default)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

pub async fn create_user(pool: &PgPool, user: User) -> anyhow::Result<User> {
    let result: User = query!(
        r#"INSERT INTO users ( user ) VALUES ( $1 ) RETURNING *"#,
        Json(user) as _
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}
