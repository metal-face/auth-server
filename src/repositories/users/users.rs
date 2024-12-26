use crate::models::user::User;
use crate::models::user_dto::UserDTO;
use argon2::{Argon2, PasswordHasher};
use password_hash::rand_core::OsRng;
use password_hash::{Salt, SaltString};
use sqlx::postgres::PgPool;
use sqlx::{query, query_as, Row};

pub async fn create_user(pool: &PgPool, user: UserDTO) -> anyhow::Result<User, anyhow::Error> {
    let password = user.password.unwrap_or_default();
    let salt_string = SaltString::generate(&mut OsRng);
    let salt: Salt = salt_string.as_salt();

    let argon2 = Argon2::default();
    let hash = argon2.hash_password(password.as_bytes(), salt).unwrap();

    let result = query(
            "INSERT INTO users ( first_name, last_name, email, hashed_password) VALUES ( $1, $2, $3, $4 ) RETURNING id, first_name, last_name, email, hashed_password, created_at, updated_at, deleted_at",
        )
            .bind(user.first_name.clone())
            .bind(user.last_name.clone())
            .bind(user.email.clone())
            .bind(hash.serialize().to_string())
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
    let result = query_as::<_, User>("SELECT u.id as id, u.first_name, u.last_name, u.email, u.hashed_password, u.created_at, u.updated_at, u.deleted_at FROM users as u WHERE email = $1")
        .bind(email)
        .fetch_one(pool)
        .await?;

    Ok(result)
}
