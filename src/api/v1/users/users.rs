use crate::repositories::users::users::User;
use crate::services::users::users::validate_user;
use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

// TODO: figure out proper return type
pub async fn create_user(
    Json(CreateUser {
        first_name,
        last_name,
        email,
        password,
    }): Json<CreateUser>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let user = User {
        first_name,
        last_name,
        email,
        password: Some(password),
        created_at: Local::now(),
        updated_at: Local::now(),
        deleted_at: None,
    };

    match validate_user(user, &state.db).await {
        Ok(user) => json!({ "status": 200, "data": {"user": user}, "success": true }),
        Err(err) => json!({ "error": err.to_string()}),
    }
}
