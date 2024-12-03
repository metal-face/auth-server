use crate::services::users::users::validate_user;
use crate::AppState;
use axum::http::StatusCode;
use axum::{
    extract::{Extension, Path},
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateUser {
    email: String,
    password: String,
}

pub async fn create_user(Json(payload): Json<CreateUser>, state: Arc<AppState>) -> Json<Value> {
    let is_valid = validate_user(payload.email, payload.password)
        .await
        .unwrap();

    if !is_valid {
        return Json(json!({
            "status": "Bad Request",
            "message": "Invalid email or password",
            "status_code": 400
        }));
    }

    Json(json!({
        "status": "success",
        "status_code": 200
    }))
}
