use crate::repositories::users::users::{get_user_by_email, User};
use argon2::Argon2;
use password_hash::{PasswordHash, PasswordVerifier};
use sqlx::PgPool;

pub async fn validate_user_credentials(
    email: String,
    password: String,
    pool: &PgPool,
) -> anyhow::Result<User> {
    let db_user = get_user_by_email(pool, &email).await?;
    let password_hash = PasswordHash::new(&db_user.hashed_password);
    let is_okay = Argon2::default()
        .verify_password(password.as_bytes(), &password_hash.unwrap())
        .is_ok();

    if is_okay {
        Ok(db_user)
    } else {
        Err(anyhow::anyhow!("Error verifying password!"))
    }
}
