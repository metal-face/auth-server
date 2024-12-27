use crate::models::user::User;
use chrono::{DateTime, Local};
use uuid::Uuid;

pub fn generate_user(
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
    created_at: DateTime<Local>,
    updated_at: DateTime<Local>,
    deleted_at: Option<DateTime<Local>>,
) -> User {
    User {
        id,
        first_name,
        last_name,
        email,
        hashed_password: password,
        created_at,
        updated_at,
        deleted_at,
    }
}
