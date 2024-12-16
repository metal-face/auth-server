use argon2::{Argon2, PasswordHasher};
use axum::response::IntoResponse;
use chrono::{DateTime, Local};
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordVerifier, Salt, SaltString};
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

fn verify_password(password: &str, hash: PasswordHash) -> Result<bool, bool> {
    match Argon2::default().verify_password(password.as_bytes(), &hash) {
        Ok(_) => Ok(true),
        Err(_) => Err(false),
    }
}

pub async fn create_user(pool: &PgPool, user: User) -> anyhow::Result<User, anyhow::Error> {
    let Some(password) = &user.password;

    let salt_string = SaltString::generate(&mut OsRng);
    let salt: Salt = salt_string.try_into()?;

    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), salt).unwrap();

    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .expect("Invalid Password");

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
