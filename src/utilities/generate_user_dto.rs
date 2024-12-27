use crate::models::user_dto::UserDTO;
use chrono::{DateTime, Local};
use uuid::Uuid;

pub fn generate_user_dto(
    id: Option<Uuid>,
    first_name: String,
    last_name: String,
    email: String,
    password: Option<String>,
    created_at: Option<DateTime<Local>>,
    updated_at: Option<DateTime<Local>>,
    deleted_at: Option<DateTime<Local>>,
) -> UserDTO {
    UserDTO {
        id,
        first_name,
        last_name,
        email,
        password,
        created_at,
        updated_at,
        deleted_at,
    }
}
