use crate::repositories::users::users::User;
use chrono::Local;
use sqlx::PgPool;

pub async fn validate_user_credentials(
    email: String,
    password: String,
    pool: &PgPool,
) -> anyhow::Result<User> {
    let temp = User {
        first_name: String::from(""),
        last_name: String::from(""),
        email: String::from(""),
        password: Default::default(),
        created_at: Local::now(),
        updated_at: Local::now(),
        deleted_at: None,
    };
    Ok(temp)
}
