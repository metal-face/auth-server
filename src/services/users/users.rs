use crate::repositories::users::users::{create_user, User};
use sqlx::PgPool;

pub async fn validate_user(user: User, pool: &PgPool) -> anyhow::Result<User, anyhow::Error> {
    let res = create_user(pool, user).await?;

    Ok(res)
}
