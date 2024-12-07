use chrono::{DateTime, Local};
use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::query;

#[derive(Default, Serialize)]
pub struct User {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: Option<String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

pub async fn create_user(pool: &PgPool, user: User) -> anyhow::Result<User, anyhow::Error> {
    let result: User = query!(
        r#"INSERT INTO users ( user ) VALUES ( $1 ) RETURNING *"#,
        Json(user) as User,
    )
    .fetch_one(pool)
    .await?;

    Ok(result)
}
