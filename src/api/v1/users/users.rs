use crate::services::users::users::validate_user;
use crate::AppState;
use axum::extract::State;
use axum::http::{Response, StatusCode};
use axum::response::{IntoResponse, Json};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
    pub deleted_at: Option<DateTime<Local>>,
}

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Json(CreateUser {
        first_name,
        last_name,
        email,
        password,
    }): Json<CreateUser>,
) -> impl IntoResponse {
    let user = UserDTO {
        first_name,
        last_name,
        email,
        password,
        created_at: Local::now(),
        updated_at: Local::now(),
        deleted_at: None,
    };

    match validate_user(user, &state.db).await {
        Ok(user) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Json(user).into_response())
            .unwrap()),
        Err(err) => Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(err.to_string().into_response())
            .unwrap()),
    }
}
