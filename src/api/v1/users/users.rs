use crate::repositories::users::users::User;
use crate::services::users::users::validate_user;
use crate::AppState;
use axum::extract::State;
use axum::http::{Response, StatusCode};
use axum::response::{IntoResponse, Json};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    first_name: String,
    last_name: String,
    email: String,
    password: String,
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
    let user = User {
        first_name,
        last_name,
        email,
        hashed_password: password,
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
