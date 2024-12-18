use crate::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use std::sync::Arc;

pub struct Credentials {
    email: String,
    password: String,
}

pub async fn log_in(
    State(state): State<Arc<AppState>>,
    Json(Credentials { email, password }): Json<Credentials>,
) -> impl IntoResponse {
}
