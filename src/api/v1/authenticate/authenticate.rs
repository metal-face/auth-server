use crate::services::authenticate::authenticate::validate_user_credentials;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
pub struct Credentials {
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    first_name: String,
    last_name: String,
    email: String,
    iat: u64,
    exp: u64,
}

#[axum::debug_handler]
pub async fn log_in(
    State(state): State<Arc<AppState>>,
    Json(Credentials { email, password }): Json<Credentials>,
) -> impl IntoResponse {
    match validate_user_credentials(email, password, &state.db).await {
        Ok(user) => Ok(Response::builder()
            .status(StatusCode::OK)
            .body(Json(user).into_response())
            .unwrap()),
        Err(err) => Err(Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(err.to_string().into_response())
            .unwrap()),
    }
}
