use chrono::{DateTime, Local};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize, sqlx::FromRow)]
pub struct AccessToken {
    pub id: Uuid,
    pub token: String,
    pub user_id: Uuid,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub expires_at: DateTime<Local>,
}
