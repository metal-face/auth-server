use chrono::{DateTime, Local};
use sqlx::postgres::PgPool;
use sqlx::types::Json;
use sqlx::{query, Error};

pub struct User {
    first_name: String,
    last_name: String,
    email: String,
    password: Option<String>,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    deleted_at: Option<DateTime<Local>>,
}

pub async fn create_user(pool: &PgPool, user: User) -> Result<User, Error> {
    let result: User = query!(
        r#"INSERT INTO users ( user ) VALUES ( $1 ) RETURNING *"#,
        Json(user) as _ 
    )
    .fetch_one(pool)
    .await?;

    return Ok(result);
}
