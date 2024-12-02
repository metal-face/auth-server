use std::sync::Arc;
use axum::{
    Json,
    extract::{ Extension, Path},
};
use axum::http::StatusCode;
use serde::Deserialize;
use serde_json::{Value, json};
use crate::AppState;
use crate::services::users::users::validate_user;

#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>, state: Arc<AppState>) -> Json<Value> {
    let is_valid = validate_user(payload.email, payload.password).await.unwrap();

    if !is_valid {
        return Json(json!({
            "status": "error",
            "message": "Invalid email or password",
            "status_code": 400
        }))
    }

    Json(json!({
        "status": "success",
        "status_code": 200
    }))
}