use crate::repositories::users::users::User;
use chrono::{DateTime, Local};
use sqlx::PgPool;

pub async fn validate_user(email: String, password: String, pool: &PgPool) -> anyhow::Result<User> {
    let user = Default::default();
    Ok(user)
}
