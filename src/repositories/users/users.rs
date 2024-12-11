use chrono::{DateTime, Local};
use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::query;

#[derive(sqlx::FromRow, Serialize)]
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
    query(
        "INSERT INTO users ( first_name, last_name, email, password, created_at, updated_at ) VALUES ( $1, $2, $3, $4, $5, $6 ) RETURNING *",
    )
        .bind(user.first_name.clone())
        .bind(user.last_name.clone())
        .bind(user.email.clone())
        .bind(user.password.clone())
        .bind(user.created_at.clone())
        .bind(user.updated_at.clone())
        .execute(pool)
        .await?;

    Ok(user)
}
