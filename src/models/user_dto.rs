use crate::models::user::User;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: Option<String>,
    pub created_at: Option<DateTime<Local>>,
    pub updated_at: Option<DateTime<Local>>,
    pub deleted_at: Option<DateTime<Local>>,
}

impl From<User> for UserDTO {
    fn from(value: User) -> Self {
        UserDTO {
            id: Some(value.id),
            first_name: value.first_name,
            last_name: value.last_name,
            email: value.email,
            password: None,
            created_at: Some(value.created_at),
            updated_at: Some(value.updated_at),
            deleted_at: value.deleted_at,
        }
    }
}
