use crate::api::v1::users::users::UserDTO;
use argon2::{Argon2, PasswordHasher};
use chrono::{DateTime, Local};
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHash, PasswordVerifier, Salt, SaltString};
use serde::Serialize;
use sqlx::postgres::PgPool;
use sqlx::{query, query_as, Row};

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

pub fn verify_password(password: &str, hash: PasswordHash) -> bool {
    Argon2::default()
        .verify_password(password.as_bytes(), &hash)
        .is_ok()
}

pub async fn create_user(pool: &PgPool, user: UserDTO) -> anyhow::Result<User, anyhow::Error> {
    let salt_string = SaltString::generate(&mut OsRng);
    let salt: Salt = salt_string.as_salt();

    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(user.password.as_bytes(), salt)
        .unwrap();

    let result = query(
            "INSERT INTO users ( first_name, last_name, email, hashed_password, created_at, updated_at ) VALUES ( $1, $2, $3, $4, $5, $6 ) RETURNING (id, first_name, last_name, email, hashed_password, created_at, updated_at, deleted_at)",
        )
            .bind(user.first_name.clone())
            .bind(user.last_name.clone())
            .bind(user.email.clone())
            .bind(hash.serialize().to_string())
            .bind(user.created_at.clone())
            .bind(user.updated_at.clone())
            .fetch_one(pool)
            .await?;

    let user = User {
        id: result.get("id"),
        first_name: result.get("first_name"),
        last_name: result.get("last_name"),
        email: result.get("email"),
        hashed_password: result.get("hashed_password"),
        created_at: result.get("created_at"),
        updated_at: result.get("updated_at"),
        deleted_at: result.get("deleted_at"),
    };

    Ok(user)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, anyhow::Error> {
    let result = query_as::<_, User>("SELECT u.first_name, u.last_name, u.email, u.hashed_password, u.created_at, u.updated_at, u.deleted_at FROM users as u WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(result)
}
