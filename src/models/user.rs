use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

#[derive(sqlx::FromRow, Serialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hashed_password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}
