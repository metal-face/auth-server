use crate::services::users::users::validate_user;
use crate::AppState;
use anyhow_http::response::{HttpJsonResult, HttpResult};
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}

pub async fn create_user(
    Json(CreateUser { email, password }): Json<CreateUser>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    match validate_user(email, password, &state.db) {
        Ok(user) => json!({ "status": 200, "user": user}),
        Err(err) => json!({ "error": err.to_string()}),
    }
}
